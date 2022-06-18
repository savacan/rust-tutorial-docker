fn main() {
    println!("Hello, world!");
    let points = vec![vec![0,0],vec![2,2],vec![3,10],vec![5,2],vec![7,0]];
    Solution::min_cost_connect_points(points);
}

#[derive(Debug)]
struct Score {
    point: Vec<i32>,
    score: i32,
}


impl Clone for Score {
    fn clone(&self) -> Self {
        Self {
            point: self.point.clone(),
            score: self.score,
        }
    }
}

struct Solution {}

impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let mut points = points;
        let first = points.pop().unwrap();
        let mut score_list: Vec<Score> = points.iter().map(|e| Score {
            point: e.clone(),
            score: Solution::calc_score(e, &first)
        }).collect();
        Solution::connect(&mut score_list)
    }

    pub fn connect(score_list: &mut Vec<Score>) -> i32 {
        if score_list.len() == 0 { return 0 }
        score_list.sort_by(|a, b| b.score.cmp(&a.score));
        let next = score_list.pop().unwrap();
        let mut score_list_from_next: Vec<Score> = score_list.iter().map(|s| {
            let next_score = Solution::calc_score(&next.point, &s.point);
            Score { point: s.point.clone(), score: if next_score > s.score { s.score } else { next_score } }
        }).collect();
        next.score + Solution::connect(&mut score_list_from_next)
    }

    pub fn calc_score(a: &Vec<i32>, b: &Vec<i32>) -> i32 {
        (a[0] - b[0]).abs() + (a[1] - b[1]).abs()
    }
}