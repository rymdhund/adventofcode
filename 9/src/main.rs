use std::cmp::min;
use std::cmp::max;
use std::collections::HashSet;
use std::collections::HashMap;
use std::iter::FromIterator;

fn min_dist1(map: &Vec<Vec<usize>>, place: usize, places_left: HashSet<usize>) -> usize {
    if places_left.is_empty() {
        return 0;
    }
    let mut min_dist = std::usize::MAX;

    for &next in places_left.iter() {
        let mut pl = places_left.clone();
        pl.remove(&next);
        let d = map[place][next] + min_dist1(map, next, pl);
        min_dist = min(min_dist, d);
    }
    min_dist
}

fn min_dist(map: &Vec<Vec<usize>>) -> usize {
    let mut min_dist = std::usize::MAX;
    let places = HashSet::<usize>::from_iter(0..map.len());

    for &start in places.iter() {
        let mut pl = places.clone();
        pl.remove(&start);
        let d = min_dist1(map, start, pl);
        min_dist = min(min_dist, d);
    }
    min_dist
}

#[test]
fn test_min_dist() {
    let map = vec![vec![0, 464, 518],
                   vec![464, 0, 141],
                   vec![518, 141, 0]];
    assert_eq!(605, min_dist(&map));
}

fn max_dist1(map: &Vec<Vec<usize>>, place: usize, places_left: HashSet<usize>) -> usize {
    if places_left.is_empty() {
        return 0;
    }
    let mut max_dist = 0;

    for &next in places_left.iter() {
        let mut pl = places_left.clone();
        pl.remove(&next);
        let d = map[place][next] + max_dist1(map, next, pl);
        max_dist = max(max_dist, d);
    }
    max_dist
}

fn max_dist(map: &Vec<Vec<usize>>) -> usize {
    let mut max_dist = 0;
    let places = HashSet::<usize>::from_iter(0..map.len());

    for &start in places.iter() {
        let mut pl = places.clone();
        pl.remove(&start);
        let d = max_dist1(map, start, pl);
        max_dist = max(max_dist, d);
    }
    max_dist
}

fn parse(input: &str) -> Vec<Vec<usize>> {
    // this got really ugly...
    let mut places: Vec<&str> = Vec::new();
    let mut tmp_map: HashMap<(&str,&str),usize> = HashMap::new();

    for line in input.lines() {
        let x: Vec<&str> = line.split(" = ").collect();
        let p: Vec<&str> = x[0].split(" to ").collect();
        let (place1, place2) = (p[0], p[1]);
        if !places.contains(&place1) {
            places.push(place1);
        }
        if !places.contains(&place2) {
            places.push(place2);
        }
        tmp_map.insert((place1,place2), x[1].parse().unwrap());
        tmp_map.insert((place2,place1), x[1].parse().unwrap());
    }

    // build the two-dimensional array
    let mut map = Vec::new();
    let mut line = Vec::new();
    for _ in 0..places.len() {
        line.push(0);
    }
    for _ in 0..places.len() {
        map.push(line.clone());
    }

    for (i, &p1) in places.iter().enumerate() {
        for (j, &p2) in places.iter().enumerate() {
            if i == j {
                map[j][i] = 0;
            } else {
                map[j][i] = *tmp_map.get(&(p2,p1)).unwrap();
                map[i][j] = *tmp_map.get(&(p2,p1)).unwrap();
            }
        }
    }
    map
}

fn part1(input: &str) -> usize {
    let map = parse(input);
    min_dist(&map)
}

fn part2(input: &str) -> usize {
    let map = parse(input);
    max_dist(&map)
}

fn main() {
    let input = include_str!("../input.txt");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}
