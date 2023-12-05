//use std::fs::File;
//use std::io::BufRead;
//use std::path::Path;
use nix::unistd::{sysconf, SysconfVar};

#[derive(Debug)]
struct CpuStat {
    name: String,
    user: u64,
    nice: u64,
    system: u64,
    idle: u64,
    iowait: u64,
    irq: u64,
    softirq: u64,
    steal: u64,
    guest: u64,
    guest_nice: u64,
}

#[derive(Debug)]
struct ProcStat {
    cpu_total: CpuStat,
    cpu_individual: Vec<CpuStat>,
    interrupts: Vec<u64>,
    context_switches: u64,
    boot_time: u64,
    processes: u64,
    processes_running: u64,
    processes_blocked: u64,
    softirq: Vec<u64>,
}

impl ProcStat {
    fn new() -> ProcStat {
        ProcStat {
            cpu_total: CpuStat::new(),
            cpu_individual: vec![],
            interrupts: vec![],
            context_switches: 0,
            boot_time: 0,
            processes: 0,
            processes_running: 0,
            processes_blocked: 0,
            softirq: vec![],
        }
    }
}

impl CpuStat {
    fn new() -> CpuStat {
        CpuStat {
            name: "".to_string(),
            user: 0,
            nice: 0,
            system: 0,
            idle: 0,
            iowait: 0,
            irq: 0,
            softirq: 0,
            steal: 0,
            guest: 0,
            guest_nice: 0,
        }
    }
}

fn generate_cpu_states(proc_stat_cpu_line: &str) -> CpuStat
{
    // Note: some of the last cpu states must be changed to unwrap_or_default()
    // for earlier linux kernel versions.
    //
    // Note: time in jiffies, must be divided by CONFIG_HZ to show time.
    let clock_time = sysconf(SysconfVar::CLK_TCK).unwrap_or(Some(1)).unwrap_or(1) as u64;
    println!("{:?}", clock_time);

    let mut splitted = proc_stat_cpu_line.split_whitespace();
    CpuStat {
        name: splitted.next().unwrap().to_string(),
        user: ((splitted.next().unwrap().parse::<u64>().unwrap()*1000_u64)/clock_time),
        nice: ((splitted.next().unwrap().parse::<u64>().unwrap()*1000_u64)/clock_time),
        system: ((splitted.next().unwrap().parse::<u64>().unwrap()*1000_u64)/clock_time),
        idle: ((splitted.next().unwrap().parse::<u64>().unwrap()*1000_u64)/clock_time),
        iowait: ((splitted.next().unwrap_or_default().parse::<u64>().unwrap()*1000_u64)/clock_time),
        irq: ((splitted.next().unwrap_or_default().parse::<u64>().unwrap()*1000_u64)/clock_time),
        softirq: ((splitted.next().unwrap_or_default().parse::<u64>().unwrap()*1000_u64)/clock_time),
        steal: ((splitted.next().unwrap_or_default().parse::<u64>().unwrap()*1000_u64)/clock_time),
        guest: ((splitted.next().unwrap_or_default().parse::<u64>().unwrap()*1000_u64)/clock_time),
        guest_nice: ((splitted.next().unwrap_or_default().parse::<u64>().unwrap()*1000_u64)/clock_time),
    }
}
fn parse_proc_stat(
    proc_stat: &str,
) -> String
{
    let mut procstat = ProcStat::new();
    for line in proc_stat.lines()
    {
        match line
        {
            line if line.starts_with("cpu ") => {
                procstat.cpu_total = generate_cpu_states(&line);
            },
            line if line.starts_with("cpu") && line.chars().nth(3) != Some(' ') => {
                procstat.cpu_individual.push(generate_cpu_states(&line));
            },
            _  => {
                //println!("{}", line);

            },
        }
    }
    format!("{:#?}", procstat)
}

fn main() {
    let lines = "cpu  506 0 512 57924 184 0 2 0 0 0
cpu0 108 0 92 9597 21 0 1 0 0 0
cpu1 94 0 76 9648 40 0 0 0 0 0
cpu2 76 0 66 9696 24 0 0 0 0 0
cpu3 91 0 98 9630 39 0 0 0 0 0
cpu4 37 0 84 9706 30 0 0 0 0 0
cpu5 98 0 94 9644 27 0 0 0 0 0
softirq 49934 30 3840 2 1309 11 0 206 7439 0 37097";

    let r = parse_proc_stat(lines);
    println!("{}",r);

}

