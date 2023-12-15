type PoweredLens = (String, usize);
type PoweredLensBox = Vec<PoweredLens>;
type PoweredLensBoxes = Vec<PoweredLensBox>;

pub fn sequence_results(input:&str) -> usize {
    input.split(",")
        .map(sequence_string)
        .sum()
}

pub fn sequence_boxes(input:&str) -> usize {
    let commands:Vec<&str> = input.split(",").collect();
    let mut boxes:PoweredLensBoxes = vec![Vec::new(); 256];
    for command in commands {
        let mut cmd = String::from("");
        let mut action = ' ';
        let mut power = 0;
        for c in command.chars() {
            match c {
                '-' | '=' => action = c,
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => power = c.to_digit(10).unwrap() as usize,
                _ => cmd += &c.to_string()
            };
        }

        match action {
            '=' => insert_lens(&cmd, power, &mut boxes),
            '-' => remove_lens(&cmd, &mut boxes),
            _   => todo!()
        }
    }

    boxes.iter().enumerate()
        .map(|(box_num, boxx)|boxx.clone().into_iter().enumerate()
                .map(|(i, (_, power))|(box_num + 1) * (i + 1) * power)
                .sum::<usize>())
        .sum()
}

fn insert_lens(hash:&str, power:usize, boxes:&mut PoweredLensBoxes) {
    let box_num = sequence_string(hash);
    for (i, (idx, _)) in boxes[box_num].clone().into_iter().enumerate() {
        if idx == hash {
            boxes[box_num][i] = (hash.to_string(), power);
            return;
        }
    }

    boxes[box_num].push((hash.to_string(), power));
}

fn remove_lens(hash:&str, boxes:&mut PoweredLensBoxes) {
    let box_num = sequence_string(hash);
    for (i, (idx, _)) in boxes[box_num].clone().into_iter().enumerate() {
        if idx == hash {
            boxes[box_num].remove(i);
        }
    }
}

fn sequence_string(input:&str) -> usize {
    input.chars()
        .map(|c|c as usize)
        .fold(0, |total,next| ((total + next) * 17) % 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_boxes() {
        assert_eq!(145, sequence_boxes("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"));
    }

    #[test]
    fn test_sequence_results() {
        assert_eq!(200, sequence_results("H"));
        assert_eq!(153, sequence_results("HA"));
        assert_eq!(1320, sequence_results("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"));
    }
}