// anchor/programs/anaheim/src/test_post_account.rs
#[cfg(test)]
mod tests {
  use super::*;
  use crate::state::post_account::PostAccount;

  #[test]
  fn test_post_size_constant() {
    assert_eq!(PostAccount::SIZE, 320); // ou la valeur attendue
  }
}
