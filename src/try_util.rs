/*
    This file is a part of alloc-perf-test.

    Copyright (C) 2024 Mohammad AlSaleh <CE.Mohammad.AlSaleh at gmail.com>
    https://github.com/MoSal

    alloc-perf-test is free software: you can redistribute it and/or modify
    it under the terms of the Affero GNU General Public License as
    published by the Free Software Foundation.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    Affero GNU General Public License for more details.

    You should have received a copy of the Affero GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

pub(crate) trait BoolExt {
    fn err_if<E, F>(self, f: F) -> Result<(), E>
        where F: FnOnce() -> E;
    fn err_if_not<E, F>(self, f: F) -> Result<(), E>
        where F: FnOnce() -> E;
}

impl BoolExt for bool {
    fn err_if<E, F>(self, f: F) -> Result<(), E>
        where F: FnOnce() -> E
    {
        match self {
            true => Err(f()),
            false => Ok(()),
        }
    }

    fn err_if_not<E, F>(self, f: F) -> Result<(), E>
        where F: FnOnce() -> E
    {
        (!self).err_if(f)
    }
}
