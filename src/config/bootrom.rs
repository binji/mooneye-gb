// This file is part of Mooneye GB.
// Copyright (C) 2014-2016 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// Mooneye GB is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Mooneye GB is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Mooneye GB.  If not, see <http://www.gnu.org/licenses/>.
use crc::crc32;
use podio::ReadPodExt;
use std::convert::From;
use std::env;
use std::fmt;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

use gameboy::BOOTROM_SIZE;
use config::{Model, DEFAULT_MODEL_PRIORITY};

#[derive(Debug)]
pub struct Bootrom {
  pub model: Model,
  pub data: Vec<u8>,
}

impl Bootrom {
  pub fn from_path(path: &Path) -> Result<Bootrom, BootromError> {
    let mut file = try!(File::open(path));
    let data = try!(file.read_exact(BOOTROM_SIZE));
    Bootrom::from_data(data)
  }
  pub fn from_data(data: Vec<u8>) -> Result<Bootrom, BootromError> {
    let checksum = crc32::checksum_ieee(&data);
    let model = try!(match checksum {
      0xc2f5cc97 => Ok(Model::Dmg0),
      0x59c8598e => Ok(Model::Dmg),
      0xe6920754 => Ok(Model::Mgb),
      0xec8a83b9 => Ok(Model::Sgb),
      0x53d0dd63 => Ok(Model::Sgb2),
      checksum => Err(BootromError::Checksum(checksum))
    });
    Ok(Bootrom {
      model: model,
      data: data,
    })
  }
  pub fn lookup(models: &[Model]) -> Option<Bootrom> {
    let mut candidates = vec![];
    let models = if models.is_empty() { &DEFAULT_MODEL_PRIORITY } else { models };

    if let Ok(cwd) = env::current_dir() {
      for model in models {
        candidates.push(cwd.join(model.bootrom_file_name()));
      }
    }

    if let Some(home) = env::home_dir().map(|home| home.join(".mooneye-gb")) {
      for model in models {
        candidates.push(home.join(model.bootrom_file_name()));
      }
    }

    for path in candidates {
      match Bootrom::from_path(&path) {
        Err(BootromError::Io(ref e)) if e.kind() == io::ErrorKind::NotFound => (),
        Err(BootromError::Io(ref e)) =>
          println!("Warning: Boot rom \"{}\" ({})", path.to_string_lossy(), e),
        Err(BootromError::Checksum(ref e)) =>
          println!("Warning: Boot rom \"{}\" ({})", path.to_string_lossy(), e),
        Ok(bootrom) => return Some(bootrom)
      }
    }
    None
  }
  pub fn save_to_home(&self) -> Result<(), io::Error> {
    if let Some(home) = env::home_dir().map(|home| home.join(".mooneye-gb")) {
      if let Err(e) = fs::create_dir(&home) {
        if e.kind() != io::ErrorKind::AlreadyExists {
          return Err(e);
        }
      }
      let path = home.join(self.model.bootrom_file_name());
      let mut file = try!(File::create(&path));
      return file.write_all(&self.data)
    }
    Ok(())
  }
}

#[derive(Debug)]
pub enum BootromError {
  Io(io::Error),
  Checksum(u32)
}

impl fmt::Display for BootromError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    use self::BootromError::*;
    match *self {
      Io(ref e) => write!(f, "{}", e),
      Checksum(crc32) => write!(f, "Unrecognized CRC32 checksum {}", crc32)
    }
  }
}

impl From<io::Error> for BootromError {
  fn from(e: io::Error) -> Self { BootromError::Io(e) }
}
