fn main(){
print!("enter a number");
let mut line=String::new();
let _k=std::io::stdin().read_line(&mut line).unwrap();
let num: i32=line.parse().unwrap();
let mut mun=num;
let mut c=0;
while mun>0
{
mun=mun/10;
c+=1;}

let mut y=num;
let mut sum=0;
while y>0
{
let mut you=0;
let qw=num%10;let mut o=1;
while you<c
{
o=o*qw;you+=1;}
sum=sum+o;
y=y/10;
}
if sum==num
{println!("it is an armstrong no.");
}
else{
println!("it is NOT an armstrong no.");
}}
