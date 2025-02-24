use sdql::tpch::q3::parallel::q3_rayon;
use sdql::tpch::q3::sequential::q3;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q3_works() {
        assert!(q3().is_ok());
    }

    #[test]
    fn q3_rayon_works() {
        assert!(q3_rayon().is_ok());
    }
}
