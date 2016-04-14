; This file is part of Mooneye GB.
; Copyright (C) 2014-2016 Joonas Javanainen <joonas.javanainen@gmail.com>
;
; Mooneye GB is free software: you can redistribute it and/or modify
; it under the terms of the GNU General Public License as published by
; the Free Software Foundation, either version 3 of the License, or
; (at your option) any later version.
;
; Mooneye GB is distributed in the hope that it will be useful,
; but WITHOUT ANY WARRANTY; without even the implied warranty of
; MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
; GNU General Public License for more details.
;
; You should have received a copy of the GNU General Public License
; along with Mooneye GB.  If not, see <http://www.gnu.org/licenses/>.

; This test verifies when the LY register changes from from 144 to 145.

; Verified results:
;   pass: MGB, CGB, AGS
;   fail: 
;   not checked: DMG, MGB, SGB, SGB2, CGB, AGB, AGS

.incdir "../../common"
.include "common.s"

  xor a
  ldh (<IE),a
  ldh (<IF),a
  ld a,$f0     ; make sure the LY=LYC flag never gets set
  ldh (<LYC),a

test_round1:
  wait_ly 144
  ld hl,LCDC
  res 7,(hl)   ; LCD off
  nop
  set 7,(hl)   ; LCD on
  wait_ly 143
  nops 50
  ldh a,(<LY) ; LY = 143
  ld (round1),a

test_round2:
  wait_ly 144
  ld hl,LCDC
  res 7,(hl)   ; LCD off
  nop
  set 7,(hl)   ; LCD on
  wait_ly 143
  nops 50
  ldh a,(<STAT) ; STAT = $83
  ld (round2),a

test_round3:
  wait_ly 144
  ld hl,LCDC
  res 7,(hl)
  nop
  set 7,(hl)
  wait_ly 143
  nops 51
  ldh a,(<LY) ; LY = 143
  ld (round3),a

test_round4:
  wait_ly 144
  ld hl,LCDC
  res 7,(hl)
  nop
  set 7,(hl)
  wait_ly 143
  nops 51
  ldh a,(<STAT) ; STAT = $80
  ld (round4),a

test_round5:
  wait_ly 144
  ld hl,LCDC
  res 7,(hl)
  nop
  set 7,(hl)
  wait_ly 143
  nops 214
  ldh a,(<LY) ; LY = 144
  ld (round5),a

test_round6:
  wait_ly 144
  ld hl,LCDC
  res 7,(hl)
  nop
  set 7,(hl)
  wait_ly 143
  nops 214
  ldh a,(<STAT) ; STAT = $81
  ld (round6),a

test_round7:
  wait_ly 144
  ld hl,LCDC
  res 7,(hl)
  nop
  set 7,(hl)
  wait_ly 143
  nops 215
  ldh a,(<LY) ; LY = 145
  ld (round7),a

test_round8:
  wait_ly 144
  ld hl,LCDC
  res 7,(hl)
  nop
  set 7,(hl)
  wait_ly 143
  nops 215
  ldh a,(<STAT) ; STAT = $81
  ld (round8),a

test_finish:
  ld a,(round1)
  ld b,a
  ld a,(round2)
  rla
  rla
  rla
  rla
  ld c,a
  push bc
  ld a,(round3)
  ld b,a
  ld a,(round4)
  ld c,a
  ld a,(round5)
  ld d,a
  ld a,(round6)
  ld e,a
  ld a,(round7)
  ld h,a
  ld a,(round8) 
  ld l,a
  pop af

  save_results
  assert_a 143
  assert_f $30
  assert_b 143
  assert_c $80
  assert_d 144
  assert_e $81
  assert_h 145
  assert_l $81
  jp process_results

.ramsection "Test-State" slot 2
  round1 db
  round2 db
  round3 db
  round4 db
  round5 db
  round6 db
  round7 db
  round8 db
.ends
