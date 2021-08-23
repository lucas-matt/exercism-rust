use std::collections::HashMap;
use crate::Result::{Win, Lose, Draw};
use std::cmp::Ordering::Equal;

struct Match<'a>(&'a str, &'a str, Result);

struct Team<'a> {
    name: &'a str,
    won: u16,
    draw: u16,
    lost: u16
}

struct League<'a> {
    teams: HashMap<&'a str, Team<'a>>
}

enum Result { Win, Lose, Draw }

impl<'a> League<'a> {
    fn new() -> Self {
        League { teams: HashMap::new() }
    }

    fn add_match(&mut self, play: Match<'a>) {
        let updates = match play {
            Match(team1, team2, Win) => [(team1, Win), (team2, Lose)],
            Match(team1, team2, Lose) => [(team1, Lose), (team2, Win)],
            Match(team1, team2, Draw) => [(team1, Draw), (team2, Draw)],
        };
        for update in updates {
            self.update_team(update);
        }
    }

    fn update_team(&mut self, update:(&'a str, Result)) {
        let name = update.0;
        let team = self.teams.entry(name).or_insert(Team {
            name,
            won: 0,
            draw: 0,
            lost: 0
        });
        match update {
            (_, Win) => team.won += 1,
            (_, Draw) => team.draw += 1,
            (_, Lose) => team.lost += 1
        }
    }

    fn ranking(&self) -> Vec<&Team> {
        let mut teams:Vec<&Team> = self.teams.values().collect();
        teams.sort_by(|&x, &y| match x.score().cmp(&y.score()) {
            Equal => y.name.cmp(&x.name),
            cmp => cmp
        });
        teams.reverse();
        teams
    }

}

impl<'a> Team<'a> {
    fn played(&self) -> u16 {
        self.won + self.lost + self.draw
    }

    fn score(&self) -> u16 {
        self.won * 3 + self.draw
    }
}

impl <'a> ToString for League<'a> {
    fn to_string(&self) -> String {
        let mut output:Vec<String> = vec!("Team                           | MP |  W |  D |  L |  P".to_string());
        for team in self.ranking() {
            output.push(team.to_string());
        }
        output.join("\n")
    }
}

impl<'a> ToString for Team<'a> {
    fn to_string(&self) -> String {
        format!("{:31}|  {} |  {} |  {} |  {} |  {}", self.name, self.played(), self.won, self.draw, self.lost, self.score())
    }
}

pub fn tally(match_results: &str) -> String {
    let scores = parse(match_results);
    let mut league = League::new();
    for play in scores {
        league.add_match(play);
    }
    league.to_string()
}

fn parse(results:&str) -> Vec<Match> {
    results
        // split up file
        .split("\n")
        .filter(|&ln| !ln.is_empty())
        .map(|ln| match ln.split(";").collect::<Vec<&str>>().as_slice() {
            &[team1, team2, "win"] => Match(team1, team2, Win),
            &[team1, team2, "loss"] => Match(team1, team2, Lose),
            &[team1, team2, "draw"] => Match(team1, team2, Draw),
            _ => panic!("Malformed file at {}", ln)
        })
        .collect()
}
