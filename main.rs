//importing libraries

extern crate csv;
use std::collections::HashMap;

//function to read csv file

#[derive(Debug)]
struct DataFrame {
   userid: Vec<String>,
   playlist: Vec<String>,
   artist: Vec<String>,
 }

impl DataFrame {
    fn new() -> DataFrame {
        DataFrame {
            userid: Vec::new(),
            playlist: Vec::new(),
            artist: Vec::new(),
        }
     }

     fn read_csv(filepath: &str, has_headers: bool) -> DataFrame {
         let file = std::fs::File::open(filepath).unwrap();
         let mut rdr = csv::ReaderBuilder::new()
            .has_headers(has_headers)
            .from_reader(file);

         let mut data_frame = DataFrame::new();

         for result in rdr.records().into_iter() {
            let record = result.unwrap();
            data_frame.push(&record);
         }
         return data_frame;
      }

      fn push(&mut self, row: &csv::StringRecord) {
          // get name
          self.userid.push(row[0].to_string());
          // get datetime
          self.playlist.push(row[1].to_string());
          // get speed
          self.artist.push(row[2].parse().unwrap());
      }
}

// Function to create 2 hash tables for our resulting vector (x,y)

#[derive(Debug, Clone)]
struct Graph {
    fwd_table: HashMap<String, u32>,
    bwd_table: HashMap<u32, String>,
    edges: Vec<(u32, u32)>, //u32
}

impl Graph {

    fn create(file: &str) -> Graph {
        let mut fh = HashMap::<String, u32>::new();
        let mut bh = HashMap::<u32, String>::new();
        let mut edges = Vec::<(u32, u32)>::new(); //u32
        let data = DataFrame::read_csv(file, true);
        let mut count = 0;
        let mut x;
        let mut y; 

        for i in 0..data.playlist.len() {
            let playlist = &data.playlist[i];
            let artist = &data.artist[i];
            //let userid = &data.userid[i];

            let entry = fh.get(playlist);
            match entry {
            None => {
                fh.insert(playlist.to_string(), count);
                bh.insert(count, playlist.to_string());
                x = count;
                count += 1; 
            }
            Some(val) => x = *val,
            }

            let entry = fh.get(artist);
            match entry{
            None => {
                fh.insert(artist.to_string(),count);
                bh.insert(count,count.to_string());
                y = count;
                count +=1;
            }
            Some(val) => y = *val,
            }
            edges.push((x,y));

        }

        return Graph{fwd_table: fh, bwd_table:bh, edges};
    }
}



// Function to 

type Vertex = usize;
type ListOfEdges = Vec<(Vertex,Vertex)>;
type AdjacencyLists = Vec<Vec<Vertex>>;

#[derive(Debug)]
struct BFS {
    n: usize, 
    outedges: AdjacencyLists,
}

fn reverse_edges(list:&ListOfEdges)
        -> ListOfEdges {
    let mut new_list = vec![];
    for (u,v) in list {
        new_list.push((*v,*u));
    }
    new_list
}

impl BFS {
    fn add_directed_edges(&mut self,
                          edges:&ListOfEdges) {
        for (u,v) in edges {
            self.outedges[*u].push(*v);
        }
    }
    fn sort_graph_lists(&mut self) {
        for l in self.outedges.iter_mut() {
            l.sort();
        }
    }
    fn create_directed(n:usize,edges:&ListOfEdges)
                                            -> BFS {
        let mut g = BFS{n,outedges:vec![vec![];n]};
        g.add_directed_edges(edges);
        g.sort_graph_lists();
        g                                        
    }
    
    fn create_undirected(n:usize,edges:&ListOfEdges)
                                            -> BFS {
        let mut g = Self::create_directed(n,edges);
        g.add_directed_edges(&reverse_edges(edges));
        g.sort_graph_lists();
        g                                        
    }
}





//Main Function 
fn main() {
    let final_results: Graph = Graph::create("Cleaned_dataset.csv"); 
    println!("{:?}", final_results);
    let mut bfs_edges = final_results.edges;
    let n = 85492;
    bfs_edges.sort();
    
    println!("{:?}", bfs_edges);
}



