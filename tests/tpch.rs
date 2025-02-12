use sdql::q3::parallel::q3_rayon;
use sdql::q3::sequential::q3;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q3_works() {
        assert!(matches!(q3(), Ok(_)));
    }

    #[test]
    fn q3_rayon_works() {
        assert!(matches!(q3_rayon(), Ok(_)));
    }
}
