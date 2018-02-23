extern crate itertools;
use itertools::Itertools;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


const MAX_BUCKET_SIZE:u16 = 65535;

#[derive(Debug)]
pub struct Bucket (Vec<ContainerType>);


// #[derive(Debug)]
// struct BucketContainer {
//     enum: ContainerType
//     data: Vec<f32>
// }

// #[derive(Debug)]
// struct Container {
//     inner: ContainerType
// }

#[derive(Debug, Clone)]
enum ContainerType {
    Direct(Vec<f32>),
    Plain(Vec<(u32,f32)>),
}

impl Bucket {
    pub fn from(data: &Vec<(u32, f32)>) -> Self {
        let mut buckets = vec![];
        for (key, group) in &data.into_iter().group_by(|elt| {elt.0 / MAX_BUCKET_SIZE as u32 }) {
        
            let bucket = (key / MAX_BUCKET_SIZE as u32) as usize;
            if buckets.len() <= bucket {
                buckets.resize(bucket+1, ContainerType::Direct(vec![]));
            }
            let data:Vec<_> = group.collect();
            if data.len() > 10_000{ // Direct
                let mut direct_data = vec![];
                for el in data {
                    if direct_data.len() <= el.0 as usize {
                        direct_data.resize(el.0 as usize+1, 0.0); 
                    }
                    direct_data[el.0 as usize] = el.1;
                }
                buckets[bucket] = ContainerType::Direct(direct_data);
            }else{
                // buckets[bucket] = ContainerType::Plain(data.iter().map(|el| (el.0 -  as u16, el.1)).collect());
                buckets[bucket] = ContainerType::Plain(data.iter().map(|el| (el.0, el.1)).collect());
            }         
        }
        Bucket(buckets)
    }

    fn get(&self, key: u32) -> Option<f32> {
        let bucket = (key / MAX_BUCKET_SIZE as u32) as usize;
        if let Some(el) = self.0.get(bucket) {
            match el {
                &ContainerType::Direct(ref data) => data.get(key as usize).cloned(),
                &ContainerType::Plain(ref data) => None
            }
        }else{
            None
        }
    }
}

