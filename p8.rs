/*
The four adjacent digits in the 1000-digit number that have the greatest product are 9 × 9 × 8 × 9 = 5832.

73167176531330624919225119674426574742355349194934
96983520312774506326239578318016984801869478851843
85861560789112949495459501737958331952853208805511
12540698747158523863050715693290963295227443043557
66896648950445244523161731856403098711121722383113
62229893423380308135336276614282806444486645238749
30358907296290491560440772390713810515859307960866
70172427121883998797908792274921901699720888093776
65727333001053367881220235421809751254540594752243
52584907711670556013604839586446706324415722155397
53697817977846174064955149290862569321978468622482
83972241375657056057490261407972968652414535100474
82166370484403199890008895243450658541227588666881
16427171479924442928230863465674813919123162824586
17866458359124566529476545682848912883142607690042
24219022671055626321111109370544217506941658960408
07198403850962455444362981230987879927244284909188
84580156166097919133875499200524063689912560717606
05886116467109405077541002256983155200055935729725
71636269561882670428252483600823257530420752963450

Find the thirteen adjacent digits in the 1000-digit number that have the greatest product. What is the value of this product?
*/

fn get_digit(digit: char) -> int
{
    match digit {
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _  => 0
    }
}

fn get_product(s: &str) -> (bool, i64)
{
    if s.contains("0") {
        // if product is going to be 0, return index of 0
        return (false, s.find('0').unwrap() as i64);
    }

    let mut prod = 1;
    for i in range(0, s.len() ) {
        prod *= get_digit(s[i] as char) as i64;
    }
    // valid product, return product itself
    return (true, prod);
}

fn main()
{
    static SLICE_LEN: uint = 13;
    let number: &'static str ="73167176531330624919225119674426574742355349194934\
                                96983520312774506326239578318016984801869478851843\
                                85861560789112949495459501737958331952853208805511\
                                12540698747158523863050715693290963295227443043557\
                                66896648950445244523161731856403098711121722383113\
                                62229893423380308135336276614282806444486645238749\
                                30358907296290491560440772390713810515859307960866\
                                70172427121883998797908792274921901699720888093776\
                                65727333001053367881220235421809751254540594752243\
                                52584907711670556013604839586446706324415722155397\
                                53697817977846174064955149290862569321978468622482\
                                83972241375657056057490261407972968652414535100474\
                                82166370484403199890008895243450658541227588666881\
                                16427171479924442928230863465674813919123162824586\
                                17866458359124566529476545682848912883142607690042\
                                24219022671055626321111109370544217506941658960408\
                                07198403850962455444362981230987879927244284909188\
                                84580156166097919133875499200524063689912560717606\
                                05886116467109405077541002256983155200055935729725\
                                71636269561882670428252483600823257530420752963450";

    let l = number.len() - SLICE_LEN;
    let mut max_prod: i64 = 0;
    let mut i = 0;
    while i <= l {
        let (status, prod) = get_product(number.slice(i, i + SLICE_LEN) );
        if false == status {
            i += prod as uint; // if there is a 0, advance up to the zero
        } else if prod > max_prod {
            max_prod = prod;
        }
        i += 1;
    }

    println!("{}", max_prod);

}

