use std::io::Read;
use std::io::stdin;
use std::io::Write;
use std::io::stdout;
use std::io;
use rand::Rng;
use std::collections::HashMap;
use std::mem;

fn main() {
    let mut input_text = String::new();
    let mut test = false;
    'outer: loop{
        if test == false{   
            println!("Enter Number of Players!");
            io::stdin()
                .read_line(&mut input_text)
                .expect("failed to read from stdin");
            test = input_text.trim().chars().all(char::is_numeric);    
            if test {            
                break 'outer;
            }
            else {
                println!("Only Numbers Allowed, Try Again.");
                input_text = String::new();
                continue 'outer;
            }
        }
        else{
            break 'outer;
        }    
    }    
    
    let player_number: u32 = input_text.trim().parse().unwrap();
    
    let mut input_player_name = String::new();
    let mut player_name = Vec::new();
    println!("Enter names of the player: ");

    for a in 1..(player_number+1){
        println!("Enter names of the player # {}: ", a);
        io::stdin()
        .read_line(&mut input_player_name)
        .expect("failed to read from stdin");

        input_player_name = input_player_name.trim().to_string();

        player_name.push(input_player_name.clone());
        input_player_name = String::new();
    }
    
    let mut max_score = 0;  
    let mut turn_number = 0;
    let mut score_record = HashMap::new();
    for a in 0..player_number{
        score_record.insert(player_name[a as usize].clone(), 0);
    }

    'outer2: loop{
        for b in 0..player_number{
            let mut dice: u32 = rand::thread_rng().gen_range(1, 7);
            
            if dice == 6{
                let six2: u32 = rand::thread_rng().gen_range(1, 7);
                if six2 == 6{
                    let six3 = rand::thread_rng().gen_range(1, 7);
                    if six3 == 6 {
                        dice = 0.clone();
                    }
                    dice = dice + six2 + six3;

                    mem::forget(six3);
                } 
                dice = dice + six2;
                
                mem::forget(six2);
            }

            let mut sum: u32 = score_record[&player_name[b as usize]] + dice;
            if sum > 100{
                sum = score_record[&player_name[b as usize]].clone();
            }
            //////////////////////////
            if max_score < sum {
                max_score = sum;
            } 
           
            for name in player_name.iter(){
                let kill = score_record[name];
               
                if sum == kill {
                    if b == 0{
                        let kicked_index = score_record.len() - 1;
                        println!("\nAlas! {} has kicked by {} at score of {}!!\n", player_name[kicked_index], player_name[b as usize], sum);
                        score_record.insert(player_name[kicked_index as usize].clone(), 0);
                        mem::forget(kicked_index);
                    }
                    else if b > 0{
                        if name.to_string() != player_name[b as usize].to_string() {
                            println!("\nAlas! {} has kicked by {} at score of {}!!\n",  name , player_name[b as usize], sum);
                            score_record.insert(name.clone(), 0);
                        }
                    }
                }
                mem::forget(kill);
            }
                
            *score_record.get_mut(&player_name[b as usize]).unwrap() = sum;
    
            println!("Turn {} Dice Roll of Player {} - {} is {} and Total {}", turn_number + 1 , b + 1 , player_name[b as usize] , dice , sum);
            
            if sum == 100{
                println!("\nPlayer {} Wins the game!!", player_name[b as usize]);
                break 'outer2;
            } 
            mem::forget(dice);
            mem::forget(sum);
        }
        pause(); //Press Enter for Next Turn
        turn_number = turn_number + 1;
        println!("");
    }
}

fn pause() {
    let mut stdout = stdout();
    stdout.flush().unwrap();
    stdin().read(&mut [10]).unwrap();
}
