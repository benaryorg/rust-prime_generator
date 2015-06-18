const LIMIT:u64=1000000;

fn main()
{
	let mut primes=vec!(2u64);

	'outer: for i in (3..LIMIT)
	{
		for n in primes.iter().take_while(|n|*n**n<=i)
		{
			if i%n==0
			{
				continue 'outer;
			}
		}
		primes.push(i);
	}

	println!("count: {}",primes.len());
}

