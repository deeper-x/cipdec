use clap::Parser;

pub struct Message {
    alph: &'static str,
    shift: i64,
}

impl Message {
    pub fn new(shift: i64) -> Self {
        Message { alph: "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZÀÁÂÃÄÅàáâãäåÈÉÊËèéêëÌÍÎÏìíîïÒÓÔÕÖòóôõöÙÚÛÜùúûü0123456789!@#$%^&*()_+-=[]{}|;:',.<>?/ ", shift: shift }
    }

    pub fn encrypt(&self, msg: String) -> Vec<i64> {
        let alph_container: Vec<char> = self.alph.chars().collect();
        let tot_alph: i64 = alph_container.len() as i64;

        let msg_container: Vec<char> = msg.chars().collect();

        let mut res: Vec<i64> = Vec::new();

        for c in msg_container {
            for (k, v) in alph_container.iter().enumerate() {
                if &c == v {
                    let k_int = k as i64;
                    if k_int > tot_alph + self.shift {
                        res.push(k_int - self.shift);
                    } else {
                        res.push(k_int + self.shift);
                    }
                }
            }
        }

        res
    }

    pub fn decrypt(&self, in_msg: &Vec<i64>, shift: i64) -> String {
        let alph_container: Vec<char> = self.alph.chars().collect();

        let mut res: String = String::from("");
        for i in in_msg {
            let idx = i - shift;
            let cur_idx = idx as usize;
            res.push(alph_container[cur_idx]);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::*;

    #[test]
    fn test_enc() {
        let obj_m = secure::cipher::Message::new(5);
        let enc = obj_m.encrypt(String::from("alberto de prezzo"));

        assert_eq!(
            enc,
            vec![5, 16, 6, 9, 22, 24, 19, 141, 8, 9, 141, 20, 22, 9, 30, 30, 19]
        );
    }

    #[test]
    fn test_dec() {
        let obj_m = secure::cipher::Message::new(5);
        let in_vec: Vec<i64> = vec![
            5, 16, 6, 9, 22, 24, 19, 141, 8, 9, 141, 20, 22, 9, 30, 30, 19,
        ];

        let dec = obj_m.decrypt(&in_vec, 5);
        assert_eq!("alberto de prezzo", dec);
    }
}
