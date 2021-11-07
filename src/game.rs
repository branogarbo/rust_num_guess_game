use rand::Rng;
use std::io;

pub struct Game {
   rounds: i64,
}

impl Game {
   pub fn new() -> Self {
      Self { rounds: 0 }
   }

   pub fn run(&mut self) {
      let rand_num = rand::thread_rng().gen_range(0..=10);
      let mut input_num: i8;

      println!("Guess a number between 0 and 10");

      loop {
         input_num = get_input_i8();

         self.rounds += 1;

         if input_num == rand_num {
            println!("Correct!");
            break;
         } else if input_num < rand_num {
            println!("Higher!");
         } else if input_num > rand_num {
            println!("Lower!");
         }
      }

      self.game_over();
   }

   fn game_over(&mut self) {
      println!("It took you {} tries!", self.rounds);
      println!("Want to play again? [Y/n]");

      let input_play_again = get_input_string();

      if input_play_again.to_ascii_lowercase() == "y" || input_play_again == "" {
         let mut new_game = Self::new();

         new_game.run();
      }
   }
}

fn get_input_string() -> String {
   let mut input = String::new();

   io::stdin().read_line(&mut input).unwrap();

   return input.trim().to_string();
}

fn get_input_i8() -> i8 {
   let i = get_input_string();

   return i.parse().unwrap();
}
