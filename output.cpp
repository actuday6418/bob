//This is a temp
#include<iostream>
#include<limits>
int main(){
std::cout<<"Begining program..\n";
float Martha;
float Peter;
std::cout<<"Enter the value of Martha\n";
while(true){std::cin>>Martha;
if(std::cin.fail()){
std::cin.clear();
std::cin.ignore(std::numeric_limits<std::streamsize>::max(), '\n');
std::cout<<"Tell Bob a decimal number!"<<std::endl;
}
else
break;
}
std::cout<<"Enter the value of Peter\n";
while(true){std::cin>>Peter;
if(std::cin.fail()){
std::cin.clear();
std::cin.ignore(std::numeric_limits<std::streamsize>::max(), '\n');
std::cout<<"Tell Bob a decimal number!"<<std::endl;
}
else
break;
}
std::cout<<"The sum of Martha and Peter is: ";
std::cout<<Peter+Martha<<std::endl;
}