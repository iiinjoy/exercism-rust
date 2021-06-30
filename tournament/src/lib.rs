use std::collections::HashMap;

#[derive(Default)]
struct TeamStat {
    wins: u32,
    draws: u32,
    losses: u32,
}

impl TeamStat {
    fn matches_played(&self) -> u32 {
        self.wins + self.draws + self.losses
    }

    fn points(&self) -> u32 {
        self.wins * 3 + self.draws
    }
}

enum MatchResult {
    Win,
    Draw,
    Loss,
}

impl From<&str> for MatchResult {
    fn from(result: &str) -> MatchResult {
        use MatchResult::*;
        match result {
            "win" => Win,
            "draw" => Draw,
            "loss" => Loss,
            _ => panic!("Unknown team result: {}", result),
        }
    }
}

pub fn tally(match_results: &str) -> String {
    let mut teams: HashMap<String, TeamStat> = HashMap::new();
    match_results.lines().for_each(|line| {
        let s = line.split(';');
        if let [team1, team2, result] = s.collect::<Vec<_>>().as_slice() {
            use MatchResult::*;
            match (*result).into() {
                Win => {
                    teams.entry(String::from(*team1)).or_default().wins += 1;
                    teams.entry(String::from(*team2)).or_default().losses += 1;
                }
                Loss => {
                    teams.entry(String::from(*team1)).or_default().losses += 1;
                    teams.entry(String::from(*team2)).or_default().wins += 1;
                }
                Draw => {
                    teams.entry(String::from(*team1)).or_default().draws += 1;
                    teams.entry(String::from(*team2)).or_default().draws += 1;
                }
            }
        }
    });
    let mut table = teams.iter().collect::<Vec<_>>();
    table.sort_by(|t1, t2| t2.1.points().cmp(&t1.1.points()).then(t1.0.cmp(t2.0)));
    let header = format!(
        "{team:30} | {MP:>2} | {W:>2} | {D:>2} | {L:>2} | {P:>2}",
        team = "Team",
        MP = "MP",
        W = "W",
        D = "D",
        L = "L",
        P = "P"
    );
    std::iter::once(header)
        .chain(table.iter().cloned().map(|(k, v)| {
            format!(
                "{team:30} | {MP:>2} | {W:>2} | {D:>2} | {L:>2} | {P:>2}",
                team = k,
                MP = v.matches_played(),
                W = v.wins,
                D = v.draws,
                L = v.losses,
                P = v.points()
            )
        }))
        .collect::<Vec<_>>()
        .join("\n")
}
