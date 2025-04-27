
#![allow(unused_variables)]
#[derive(Debug, Clone , Copy)]
struct CubeSat {
    id : u64
}
#[derive(Debug)]
struct Mailbox{
    messages : Vec<Message>
}
#[derive(Debug)]
struct Message {
    id : u64, 
    content : String
}
struct GroundStation;

impl Mailbox {
    fn post(&mut self , msg: Message){
        self.messages.push(msg);
    }
    fn deliver(&mut self , recipient : &CubeSat) -> Option<Message>{
        for i in 0..self.messages.len(){
            if self.messages[i].id == recipient.id{
                let msg = self.messages.remove(i);
                return Some(msg);
            }
        }
        None
    }
}

impl GroundStation{
    fn connect (&self  , sat_id : u64) -> CubeSat{
        CubeSat { id : sat_id}
    }

    fn send(&self , mailbox : &mut Mailbox , msg : Message){
        mailbox.post(msg)
    }
}

impl CubeSat{
    fn recv(&self , mailbox : &mut Mailbox) -> Option<Message>{
        mailbox.deliver(&self)
    }
}

fn fetch_sat_ids() -> Vec<u64>{
    vec![1,2,3]
}
fn main(){
    let mut mail = Mailbox {
        messages : vec![]
    };
    let ground_station = GroundStation{};

    let sat_ids = fetch_sat_ids();

    for sat_id in sat_ids {
        let sat = ground_station.connect(sat_id);
        let message = Message{
            id : sat_id , content : String::from("Hello")
        };
        ground_station.send(&mut mail, message);
    }

    let sat_ids = fetch_sat_ids();
    for sat_id in sat_ids{
        let sat = ground_station.connect(sat_id);
        let msg = sat.recv(&mut mail);
    }
}

