use sdql::tpch::q3::parallel::q3_rayon;
use sdql::tpch::q3::sequential::q3;
use sdql::tpch::q6::parallel::q6_rayon;
use sdql::tpch::q6::sequential::q6;

#[cfg(test)]
mod tests {
    use super::*;
    use sdql::tpch::q3::format_q3_result;
    use sdql::tpch::q6::format_q6_result;

    #[test]
    fn q3_works() {
        let result = q3("0.01").unwrap();
        let pretty = format_q3_result(&result);
        let expected = include_str!("results/tpch/SF_0.01/q3.result");
        assert_eq!(pretty, expected);
    }

    #[test]
    fn q3_rayon_works() {
        let result = q3_rayon("0.01").unwrap();
        let pretty = format_q3_result(&result);
        let expected = include_str!("results/tpch/SF_0.01/q3.result");
        assert_eq!(pretty, expected);
    }

    #[test]
    fn q6_works() {
        let result = q6("0.01").unwrap();
        let pretty = format_q6_result(&result);
        let expected = include_str!("results/tpch/SF_0.01/q6.result");
        assert_eq!(pretty, expected);
    }

    #[test]
    fn q6_rayon_works() {
        let result = q6_rayon("0.01").unwrap();
        let pretty = format_q6_result(&result);
        let expected = include_str!("results/tpch/SF_0.01/q6.result");
        assert_eq!(pretty, expected);
    }
}
