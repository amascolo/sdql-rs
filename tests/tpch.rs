use sdql::tpch::q3::parallel::q3_rayon;
use sdql::tpch::q3::sequential::q3;
use sdql::tpch::q6::parallel::q6_rayon;
use sdql::tpch::q6::sequential::q6;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q3_works() {
        assert!(q3("0.01").is_ok());
    }

    #[test]
    fn q3_rayon_works() {
        assert!(q3_rayon("0.01").is_ok());
    }

    #[test]
    fn q6_works() {
        assert!(q6("0.01").is_ok());
    }

    #[test]
    fn q6_rayon_works() {
        assert!(q6_rayon("0.01").is_ok());
    }
}
