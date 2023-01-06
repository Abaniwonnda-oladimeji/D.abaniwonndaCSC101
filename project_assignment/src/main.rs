use::std::io::Write;
fn main() {
    println!("input 7 to create file for Aigbona juliet and Akpevwe Iloka");
    println!("input 8 to create file for Adamu Sagamu and Gbenga Daniels");
    println!("input 9 to create file for Mairia Akinsola and Ehis Ero");

   
    let mut input1 = String::new();
    std::io::stdin().read_line(&mut input1).expect("could not read input");
    
    if input1.trim() == "7"{
        fn code_7(){
            let consulting = vec!["Analystics consulting services","Customer experience","Cyber security strategy risk compliance and resilience",
                "Digital transformation","Risk consulting services","Supply chain and operations","commercial strategy"];

            let assurance = vec!["Adult services","climate change and sustainability services","financial accounting advisory services",
                "Restructuring and turn around strategy","Private client Audit experience","Accounting link","technology transformation"];


            let name1 = "Aigbona juliet";
            let name2 = "AKpevwe Iloka";
            let qual1 = "B.Sc.";
            let qual2 = "HND";
            let dept1 = "consulting";
            let dept2 = "Assurance";

            let mut file = std::fs::File::create("Aigbona_juliet.txt").expect("could not open file");
            writeln!(&mut file, "This file belongs to {}\nHer qualification is {}\nShe belongs to {} 
                department,\nHer job includes {:?}",name1, qual1, dept1,consulting);
            println!("{} file created succesfully",name1);

            
            let mut file = std::fs::File::create("AKpevwe_Iloka.txt").expect("could not open file");
            writeln!(&mut file, "This file belongs to {}\nHis qualification is {}\nHe belongs to {} 
                department,\nHis job includes {:?}",name2, qual2, dept2,assurance);
            println!("{} file created succesfully",name2);
        }
        code_7();
    }
    if input1.trim() == "8"{
        fn code_8(){
            let tax = vec!["tax planning","tax functions operations","tax policies and controversies",
                "global trade ","tax accounting","tax compliance","transactiontax"]; 

            let people_and_workforce = vec!["Change Management and Experiences","HR transformation","Integrated Workforce Mobility",
                "Learning and Development Consulting","Recognition and award advisory","workforce analytics","people_and_workforce"];

            let name1 = "Adamu Sagamu";
            let name2 = "Gbenga Daniels";
            let qual1 = "B.Sc.";
            let qual2 = "HND";
            let dept1 = "tax";
            let dept2 = "people and workforce";

            let mut file = std::fs::File::create("Adamu Sagamu.txt").expect("could not open file");
            writeln!(&mut file, "This file belongs to {}\nHis qualification is {}\nhe belongs to {} department,\nHis job includes {:?}",name1, qual1, dept1,tax);
            println!("{} file created succesfully",name1);

            let mut file = std::fs::File::create("Gbenga Daniels.txt").expect("could not open file");
            writeln!(&mut file, "This file belongs to {}\nHis qualification is {}\nHe belongs to {} department,\nHis job includes {:?}",name2, qual2, dept2,people_and_workforce);
            println!("{} file created succesfully",name2);
        }
        code_8()
    }
    if input1.trim() == "9"{
        fn code_9(){
            let strategy_by_ey_pantheon = vec!["strategy consulting","corporate growth","transaction strategy and execution",
                "Restructuring and turn around strategy","industry strategy","digital business building","commercial strategy"];

            let transactions_and_corporate_finance = vec!["corporate Finance","Divestments and carveouts","sustainability and ESG services",
            "M&A Advisory","M&A Integration","M&A technology and Tools","M&A Advanced Analytics"];

            let name1 = "Ehis Ero";
            let name2 = "Maria Akinsola";
            let qual1 = "M.Sc.";
            let qual2 = "M.Sc.";
            let dept1 = "strategy_by_EY_pantheon";
            let dept2 = "Transactions_And_Corporate_Finance";

            let mut file = std::fs::File::create("Ehis Ero.txt").expect("could not open file");
            writeln!(&mut file, "This file belongs to {}\nHis qualification is {}\nHe belongs to {} department,\nHis job includes {:?}",name1, qual1, dept1,strategy_by_ey_pantheon);
            println!("{} file created succesfully",name1);

            let mut file = std::fs::File::create("Maria Akinsola.txt").expect("could not open file");
            writeln!(&mut file, "This file belongs to {}\nHer qualification is {}\nShe belongs to {} 
                department,\nHer job includes {:?}",name2, qual2, dept2,transactions_and_corporate_finance);
            println!("{} file created succesfully",name2);

        }
        code_9()
    }

}






