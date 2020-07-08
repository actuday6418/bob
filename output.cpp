//This is a temp
#include<iostream>
#include<limits>
int main(){
std::cout<<"Begining program..\n";
int Martha;
while(true){std::cin>>Martha;
if(std::cin.fail()){
std::cin.clear();
std::cin.ignore(std::numeric_limits<std::streamsize>::max(), '\n');
std::cout<<"Input a number"<<std::endl;
}
else
break;
}
std::cout<<Martha;
}
