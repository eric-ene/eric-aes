use std::vec;
use nalgebra::{matrix, Const, Matrix4, MatrixView, Vector4, U1, U4};

pub trait MatrixTools {
  type RetType;
  fn gf_mul(&self, rhs: Vector4<u8>) -> Self::RetType;
  fn gf_dot(lhs: Vector4<u8>, rhs: Vector4<u8>) -> u8;
}

pub trait GfMul {
  fn gf_mul(self, rhs: Self) -> Self;
  fn gf_mul_2(self) -> Self;
  fn gf_mul_3(self) -> Self;
}

impl MatrixTools for Matrix4<u8> {
  type RetType = Vector4<u8>;

  fn gf_mul(&self, rhs: Vector4<u8>) -> Self::RetType {

    let mut result = Vector4::zeros();

    for (index, row) in self.row_iter().enumerate() {
      let current_row: Vector4<u8> = row.transpose().into();
      result[index] = Self::gf_dot(rhs, current_row);
    }
    
    result
  }

  fn gf_dot(lhs: Vector4<u8>, rhs: Vector4<u8>) -> u8 {
    return 
      lhs[0].gf_mul(rhs[0]) ^
        lhs[1].gf_mul(rhs[1]) ^
        lhs[2].gf_mul(rhs[2]) ^
        lhs[3].gf_mul(rhs[3]);
  }
}

impl GfMul for u8 {
  fn gf_mul(self, rhs: Self) -> Self {
    match rhs {
      1 => self,
      2 => self.gf_mul_2(),
      3 => self.gf_mul_3(),
      0x09 => self.gf_mul_2().gf_mul_2().gf_mul_2() ^ self,
      0x0B => (self.gf_mul_2().gf_mul_2() ^ self).gf_mul_2() ^ self,
      0x0D => (self.gf_mul_2() ^ self).gf_mul_2().gf_mul_2() ^ self,
      0x0E => ((self.gf_mul_2() ^ self).gf_mul_2() ^ self).gf_mul_2(),
      _ => 0
    }
  }
  fn gf_mul_2(self) -> Self {
    if self & 0x80 != 0 {
      return (self << 1) ^ 0x1b
    }
    
    return self << 1
  }
  fn gf_mul_3(self) -> Self {
    return self.gf_mul_2() ^ self;
  }
}