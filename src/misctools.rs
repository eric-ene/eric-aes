use std::ops::Mul;
use nalgebra::{matrix, Matrix, Matrix4};
use crate::aestools::{CryptError, INV_SBOX, SBOX};
use crate::aestools::CryptError::WrongSize;
use crate::matrix::MatrixTools;

const RC: [u8; 10] = [
  0x01, 0x02, 0x04, 0x08,
  0x10, 0x20, 0x40, 0x80,
  0x1B, 0x36
];

const MDS_MATRIX: Matrix4<u8> = matrix![
  0x02, 0x03, 0x01, 0x01;
  0x01, 0x02, 0x03, 0x01;
  0x01, 0x01, 0x02, 0x03;
  0x03, 0x01, 0x01, 0x02;
];

const INV_MDS_MATRIX: Matrix4<u8> = matrix![
  0x0E, 0x0B, 0x0D, 0x09;
  0x09, 0x0E, 0x0B, 0x0D;
  0x0D, 0x09, 0x0E, 0x0B;
  0x0B, 0x0D, 0x09, 0x0E;
];

pub trait CryptTools {
  fn xor(&self, rhs: &Self) -> Result<Self, CryptError> where Self: Sized;
  fn sub_bytes(&self) -> Result<Self, CryptError> where Self: Sized;
  fn shift_row(&mut self, row: usize, by: usize) -> Result<(), CryptError> where Self: Sized;
  fn mix_columns(&self) -> Result<Self, CryptError> where Self: Sized;

  fn sub_bytes_inv(&self) -> Result<Self, CryptError> where Self: Sized;
  fn shift_row_inv(&mut self, row: usize, by: usize) -> Result<(), CryptError> where Self: Sized;
  fn mix_columns_inv(&self) -> Result<Self, CryptError> where Self: Sized;
}

impl CryptTools for Vec<u8> {
  fn xor(&self, rhs: &Self) -> Result<Self, CryptError> {
    if self.len() != rhs.len() { 
      return Err(CryptError::MismatchedSize((self.len(), rhs.len())))
    }
    
    let retval = self.iter()
      .zip(rhs.iter())
      .map(|(a, b)| a ^ b)
      .collect();
    
    return Ok(retval);
  }

  fn sub_bytes(&self) -> Result<Self, CryptError> {
    let retval = self.iter().map(|val| SBOX[*val as usize]).collect();
    
    return Ok(retval);
  }

  fn shift_row(&mut self, row: usize, by: usize) -> Result<(), CryptError> {
    let mut matrix = get_chunks(&self, 4)?;
    
    let mut row_to_shift = matrix[row].clone();

    row_to_shift.rotate_left(by);
    
    matrix[row] = row_to_shift;
    
    *self = flatten(&matrix);
    Ok(())
  }

  fn mix_columns(&self) -> Result<Self, CryptError> {
    if self.len() != 16 {
      return Err(CryptError::WrongSize(self.len()));
    }
    
    let matrix: Matrix4<u8> = Matrix4::<u8>::from_row_slice(&self);
    let mut mixed = vec![];
    
    for column in matrix.column_iter() {
      let mixed_column = MDS_MATRIX.gf_mul(column.into());
      mixed.push(mixed_column);
    }
    
    let mut retval = vec![];
    
    for column in mixed {
      for value in column.iter() {
        retval.push(*value)
      }
    }
    
    let final_matrix = Matrix4::<u8>::from_row_slice(&retval);
    return Ok(final_matrix.as_slice().to_vec());
  }

  fn sub_bytes_inv(&self) -> Result<Self, CryptError> {
    let retval = self.iter().map(|val| INV_SBOX[*val as usize]).collect();

    return Ok(retval);
  }

  fn shift_row_inv(&mut self, row: usize, by: usize) -> Result<(), CryptError> {
    let mut matrix = get_chunks(&self, 4)?;
    
    let mut row_to_shift = matrix[row].clone();

    row_to_shift.rotate_right(by);

    matrix[row] = row_to_shift;

    *self = flatten(&matrix);
    Ok(())
  }

  fn mix_columns_inv(&self) -> Result<Self, CryptError> {
    if self.len() != 16 {
      return Err(CryptError::WrongSize(self.len()));
    }

    let matrix: Matrix4<u8> = Matrix4::<u8>::from_row_slice(&self);
    let mut mixed = vec![];

    for column in matrix.column_iter() {
      let mixed_column = INV_MDS_MATRIX.gf_mul(column.into());
      mixed.push(mixed_column);
    }

    let mut retval = vec![];

    for column in mixed {
      for value in column.iter() {
        retval.push(*value)
      }
    }

    let final_matrix = Matrix4::<u8>::from_row_slice(&retval);
    return Ok(final_matrix.as_slice().to_vec());
  }
}

pub fn pad(input: &Vec<u8>) -> Vec<u8> {
  let mut retval = input.clone();
  
  let remaining = (16 - (input.len() % 16)) % 16;
  retval.append(&mut vec![' ' as u8; remaining]);
  
  return retval;
}

pub fn get_rcon(i: usize) -> u32 {
  let i = i - 1;
  let rcon = [RC[i], 0, 0, 0];

  return u32::from_be_bytes(rcon);
}

pub fn condense(input: &Vec<u8>) -> Vec<u32> {
  if input.len() % 4 != 0 {
    println!("INPUT KEY DOES NOT HAVE LEN MULTIPLE OF 4! RETURNING EMPTY VEC.");
    return vec![];
  }

  let mut retval = vec![];

  for i in (0..input.len()).step_by(4) {
    let bytes = [input[i], input[i+1], input[i+2], input[i+3]];
    retval.push(u32::from_be_bytes(bytes));
  }

  return retval;
}

pub fn get_chunks(input: &Vec<u8>, size: usize) -> Result<Vec<Vec<u8>>, CryptError> {
  let data_len = input.len();
  if data_len % size != 0 { return Err(WrongSize(data_len)); }
  
  let mut retval = vec![];
  
  for i in (0..data_len).step_by(size) {
    let current_chunk = input[i..i+size].to_vec();
    retval.push(current_chunk);
  }
  
  return Ok(retval);
}

pub fn flatten(input: &Vec<Vec<u8>>) -> Vec<u8> {
  let mut retval = vec![];
  
  for row in input {
    for entry in row {
      retval.push(*entry);
    }
  }
  
  return retval
}

pub fn rot_word(word: u32) -> u32 {
  return word.rotate_left(8)
}