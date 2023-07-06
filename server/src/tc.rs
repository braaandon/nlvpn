use crate::limit::{LimitType, Port};

pub fn create_qdisc() {
    std::process::Command::new("sudo")
        .args(
            "tc qdisc add dev wg0 root handle 1: prio"
                .split_whitespace()
                .collect::<Vec<&str>>(),
        )
        .status()
        .unwrap();
}

pub fn add_filter(port: Port) {
    let command: String;

    if port.start == LimitType::Character.to_port().start {
        command = format!(
            "tc filter add dev wg0 protocol ip parent 1: prio {0} basic match cmp(u16 at 0 layer transport eq {0}) or (cmp(u16 at 0 layer transport gt {0}) and cmp(u16 at 0 layer transport lt {1})) or cmp(u16 at 0 layer transport eq {1}) action police rate 100bps burst 100b",
            port.start,
            port.end
        );
    } else {
        command = format!(
            "tc filter add dev wg0 protocol ip parent 1: prio {0} basic match cmp(u16 at 0 layer transport eq {0}) or (cmp(u16 at 0 layer transport gt {0}) and cmp(u16 at 0 layer transport lt {1})) or cmp(u16 at 0 layer transport eq {1}) action police rate 1bps burst 1b",
            port.start,
            port.end
        );
    }

    std::process::Command::new("sudo")
        .args(command.split_whitespace().collect::<Vec<&str>>())
        .status()
        .unwrap();
}

pub fn delete_filter(port: Port) {
    let command = format!("tc filter del dev wg0 prio {}", port.start);

    std::process::Command::new("sudo")
        .args(command.split_whitespace().collect::<Vec<&str>>())
        .status()
        .unwrap();
}

pub fn destroy_qdisc() {
    std::process::Command::new("sudo")
        .args(
            "tc qdisc del dev wg0 root"
                .split_whitespace()
                .collect::<Vec<&str>>(),
        )
        .status()
        .unwrap();
}
