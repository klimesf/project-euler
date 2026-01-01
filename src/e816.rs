#[allow(unused)]
pub(crate) fn e816() {
    println!("{}", shortest_distance(2_000_000))
}

#[allow(unused)]
fn shortest_distance(k: usize) -> String {
    let mut rand = vec![0_u64; 2 * k + 1];
    rand[0] = 290_797;
    for i in 1..rand.len() {
        rand[i] = (rand[i - 1] * rand[i - 1]) % 50_515_093;
    }

    let mut min = u64::MAX;
    let mut points: Vec<(u64, u64)> = vec![];
    for n in 0..k {
        if n % 1_000 == 0 {
            println!("{} {:.9}", n, (min as f64).sqrt());
        }
        let p = (rand[2 * n], rand[2 * n + 1]);
        for other in &points {
            let dist = (p.0.max(other.0) - p.0.min(other.0)).pow(2) + (p.1.max(other.1) - p.1.min(other.1)).pow(2);
            min = min.min(dist);
        }
        points.push(p);
    }
    format!("{:.9}", (min as f64).sqrt())
}

#[cfg(test)]
mod e816_tests {
    use crate::e816::shortest_distance;

    #[test]
    fn shortest_distance_works() {
        assert_eq!("546446.466846479", shortest_distance(14));
        assert_eq!("", shortest_distance(2_000_000));
    }
}
