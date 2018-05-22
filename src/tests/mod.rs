use kmeans;
use gap_statistic;

#[test]
fn test_optimalk() {

    // Data with 3 clusters
    let data = [[ 8.86692184],
       [ 9.93955522],
       [ 7.36376903],
       [ 0.54320146],
       [ 7.74309779],
       [ 0.14701856],
       [ 1.84603812],
       [ 9.33217457],
       [ 8.28287226],
       [ 7.96046545],
       [ 1.74699624],
       [ 8.15508527],
       [ 9.38785074],
       [ 8.16690664],
       [ 6.39884122],
       [ 0.64011663],
       [ 2.09079509],
       [ 1.40730812],
       [ 7.22726001],
       [ 0.01822176],
       [ 7.33734737],
       [ 9.75974298],
       [ 8.03078009],
       [ 7.32594775],
       [ 6.86864567],
       [ 8.57122661],
       [ 8.97177858],
       [ 6.87631176],
       [ 8.34116162],
       [-0.24210989],
       [ 7.29633604],
       [ 1.34882931],
       [ 8.09819935],
       [-0.36227273],
       [ 8.08471621],
       [ 1.71638721],
       [ 0.4262194 ],
       [ 0.53603222],
       [ 8.723726  ],
       [ 7.57277222]];

    let data = data
        .iter()
        .map(|arr| arr.to_vec())
        .collect::<Vec<Vec<f64>>>();

    let result = gap_statistic::optimal_k(data, (1..10).collect());
    println!("Got {:?}", result)
}