use anyhow::Result;
use ndarray::Array2;
use std::fs;

pub(crate) enum Source {
    Stdin,
    Files(Vec<String>),
}

impl Source {
    /// Return correct variant for command line args.
    pub(crate) fn from(file_paths: Vec<String>) -> Self {
        if file_paths.is_empty() {
            Source::Stdin
        } else {
            Source::Files(file_paths)
        }
    }
}

type Tuple = (u8, u8);

#[derive(Debug)]
pub(crate) struct Digraph {
    delimiter: String,
    // matrix: Option<Array2<usize>>,
}

impl Digraph {
    /// Create new instance using delimiter from command line, or newline char
    /// in the case that it is none
    pub(crate) fn new(tuple_separator: Option<String>) -> Result<Self> {
        let delimiter = if tuple_separator.is_none() {
            String::from("\n")
        } else {
            tuple_separator.unwrap()
        };
        Ok(Digraph {
            delimiter,
            // matrix: None,
        })
    }
    /// Match on the source to determine if we receive stdin or an array of
    /// existing files; build matrices after processing source.
    pub(crate) fn build(&self, source: &Source, overwrite: bool) -> Result<()> {
        let mut matrices: Vec<Array2<u8>> = Vec::new();
        match (source, overwrite) {
            // TODO(nonnontrivial) implement
            (Source::Stdin, false) => unimplemented!(),
            (Source::Stdin, true) => unimplemented!(),
            (Source::Files(_paths), true) => unimplemented!(),
            (Source::Files(paths), false) => {
                for path in paths {
                    let tuples = self.parse_tuples(path)?;
                    let matrix = self.build_matrix(&tuples)?;
                    matrices.push(matrix);
                }
            }
        }
        for m in matrices {
            println!("{:0.0}", m);
        }
        Ok(())
    }
    /// Create square adjacency matrix from tuples.
    pub fn build_matrix(&self, tuples: &Vec<Tuple>) -> Result<Array2<u8>> {
        let mut dimension: usize = 0;
        for t in tuples {
            if t.0 > dimension as u8 {
                dimension = t.0 as usize + 1;
            } else if t.1 > dimension as u8 {
                dimension = t.1 as usize + 1;
            };
        }
        let mut m = Array2::zeros((dimension, dimension));
        for t in tuples {
            m[[t.0 as usize, t.1 as usize]] = 1;
        }
        Ok(m)
    }
    /// Collect all tuples in input source into vector.
    fn parse_tuples(&self, path: &String) -> Result<Vec<Tuple>> {
        let file_contents = fs::read_to_string(path)?;
        let tokens = file_contents.split(&self.delimiter);
        let mut tuples: Vec<Tuple> = Vec::new();
        for raw_token in tokens {
            if raw_token == "" {
                continue;
            }
            // TODO(nonnontrivial) find better way of parsing tuples
            let mut tuple: Tuple = (0, 0);
            let token: Vec<&str> = raw_token.split_whitespace().collect();
            for i in 0..token.len() {
                match i {
                    0 => {
                        tuple.0 = token[i].parse().unwrap();
                    }
                    _ => {
                        tuple.1 = token[i].parse().unwrap();
                    }
                }
            }
            tuples.push(tuple);
        }
        Ok(tuples)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn matrix_build() {
        let digraph = Digraph::new(None);
        let s = vec![(0, 2), (2, 0)];
        let m = digraph.unwrap().build_matrix(&s);
        assert_eq!(m.unwrap(), array![[0, 0, 1], [0, 0, 0], [1, 0, 0]]);
    }
}
