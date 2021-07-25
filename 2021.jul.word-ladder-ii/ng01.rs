/*
"hit"
"cog"
["hot","dot","dog","lot","log","cog"]
"hit"
"cog"
["hot","dot","dog","lot","log"]
"cet"
"ism"
["kid","tag","pup","ail","tun","woo","erg","luz","brr","gay","sip","kay","per","val","mes","ohs","now","boa","cet","pal","bar","die","war","hay","eco","pub","lob","rue","fry","lit","rex","jan","cot","bid","ali","pay","col","gum","ger","row","won","dan","rum","fad","tut","sag","yip","sui","ark","has","zip","fez","own","ump","dis","ads","max","jaw","out","btu","ana","gap","cry","led","abe","box","ore","pig","fie","toy","fat","cal","lie","noh","sew","ono","tam","flu","mgm","ply","awe","pry","tit","tie","yet","too","tax","jim","san","pan","map","ski","ova","wed","non","wac","nut","why","bye","lye","oct","old","fin","feb","chi","sap","owl","log","tod","dot","bow","fob","for","joe","ivy","fan","age","fax","hip","jib","mel","hus","sob","ifs","tab","ara","dab","jag","jar","arm","lot","tom","sax","tex","yum","pei","wen","wry","ire","irk","far","mew","wit","doe","gas","rte","ian","pot","ask","wag","hag","amy","nag","ron","soy","gin","don","tug","fay","vic","boo","nam","ave","buy","sop","but","orb","fen","paw","his","sub","bob","yea","oft","inn","rod","yam","pew","web","hod","hun","gyp","wei","wis","rob","gad","pie","mon","dog","bib","rub","ere","dig","era","cat","fox","bee","mod","day","apr","vie","nev","jam","pam","new","aye","ani","and","ibm","yap","can","pyx","tar","kin","fog","hum","pip","cup","dye","lyx","jog","nun","par","wan","fey","bus","oak","bad","ats","set","qom","vat","eat","pus","rev","axe","ion","six","ila","lao","mom","mas","pro","few","opt","poe","art","ash","oar","cap","lop","may","shy","rid","bat","sum","rim","fee","bmw","sky","maj","hue","thy","ava","rap","den","fla","auk","cox","ibo","hey","saw","vim","sec","ltd","you","its","tat","dew","eva","tog","ram","let","see","zit","maw","nix","ate","gig","rep","owe","ind","hog","eve","sam","zoo","any","dow","cod","bed","vet","ham","sis","hex","via","fir","nod","mao","aug","mum","hoe","bah","hal","keg","hew","zed","tow","gog","ass","dem","who","bet","gos","son","ear","spy","kit","boy","due","sen","oaf","mix","hep","fur","ada","bin","nil","mia","ewe","hit","fix","sad","rib","eye","hop","haw","wax","mid","tad","ken","wad","rye","pap","bog","gut","ito","woe","our","ado","sin","mad","ray","hon","roy","dip","hen","iva","lug","asp","hui","yak","bay","poi","yep","bun","try","lad","elm","nat","wyo","gym","dug","toe","dee","wig","sly","rip","geo","cog","pas","zen","odd","nan","lay","pod","fit","hem","joy","bum","rio","yon","dec","leg","put","sue","dim","pet","yaw","nub","bit","bur","sid","sun","oil","red","doc","moe","caw","eel","dix","cub","end","gem","off","yew","hug","pop","tub","sgt","lid","pun","ton","sol","din","yup","jab","pea","bug","gag","mil","jig","hub","low","did","tin","get","gte","sox","lei","mig","fig","lon","use","ban","flo","nov","jut","bag","mir","sty","lap","two","ins","con","ant","net","tux","ode","stu","mug","cad","nap","gun","fop","tot","sow","sal","sic","ted","wot","del","imp","cob","way","ann","tan","mci","job","wet","ism","err","him","all","pad","hah","hie","aim","ike","jed","ego","mac","baa","min","com","ill","was","cab","ago","ina","big","ilk","gal","tap","duh","ola","ran","lab","top","gob","hot","ora","tia","kip","han","met","hut","she","sac","fed","goo","tee","ell","not","act","gil","rut","ala","ape","rig","cid","god","duo","lin","aid","gel","awl","lag","elf","liz","ref","aha","fib","oho","tho","her","nor","ace","adz","fun","ned","coo","win","tao","coy","van","man","pit","guy","foe","hid","mai","sup","jay","hob","mow","jot","are","pol","arc","lax","aft","alb","len","air","pug","pox","vow","got","meg","zoe","amp","ale","bud","gee","pin","dun","pat","ten","mob"]
*/
use std::collections::{HashMap, VecDeque};

fn is_match(str1: &str, str2: &str) -> bool {
    let chars1: Vec<char> = str1.chars().collect();
    let chars2: Vec<char> = str2.chars().collect();
    for index in 0..chars1.len() {
        let mut A = chars1.clone();
        A[index] = '*';
        let mut B = chars2.clone();
        B[index] = '*';
        if A == B {
            return true
        }
    }
    
    false
}

impl Solution {
    pub fn find_ladders(begin_word: String, end_word: String, mut word_list: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
        word_list.push(begin_word);
        if let Some(goal_index) = word_list.iter().position(|word| *word == end_word) {
            // word_list.push(end_word);
            (0..word_list.len()).for_each(|index| {
                (0..word_list.len()).for_each(|target_index| {
                    if index != target_index {                    
                        if is_match(&word_list[index], &word_list[target_index]) {
                            map.entry(index).and_modify(|c| c.push(target_index)).or_insert(vec![target_index]);
                        }
                    }
                });
            });

            let mut queue: VecDeque<Vec<usize>> = VecDeque::new();
            queue.push_back(vec![word_list.len() - 1]);

            while !queue.is_empty() {
                let mut result: Vec<Vec<usize>> = Vec::new();
                let mut next_queue: VecDeque<Vec<usize>> = VecDeque::new();
                queue.into_iter().for_each(|mut q| {
                    let index = q.last().unwrap();
                    let cand = map.get(index).unwrap();
                    // println!("cand: {:?}", cand);
                    cand.iter().for_each(|next| {
                        // println!("q: {:?}, next: {}, contains: {}", q, next, q.contains(&*next));
                        if !q.contains(&*next) {
                            let mut next_q = q.clone();
                            next_q.push(*next);
                            if *next == goal_index {
                                result.push(next_q.clone());
                            }
                            next_queue.push_back(next_q);
                        }
                    });
                    // println!("{:?}", next_queue);
                });
                queue = next_queue;
                if !result.is_empty() {
                    // println!("result {:?}", result);
                    return result.into_iter().map(|r| {
                        r.into_iter().map(|index| word_list[index].clone()).collect::<Vec<String>>()
                    }).collect()
                }
            }

            // println!("{:?}", word_list);
            // println!("{:?}", map);
        }

        vec![]        
    }
}