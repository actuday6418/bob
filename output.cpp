//This is a temp
#include<iostream>
#include<limits>
int main(){
std::cout<<"Enter the Radius of the circle in metres: ";
float Radius;
while(true){std::cin>>Radius;
if(std::cin.fail()){
std::cin.clear();
std::cin.ignore(std::numeric_limits<std::streamsize>::max(), '\n');
std::cout<<"Tell Bob a decimal number!"<<std::endl;
}
else
break;
}
std::cout<<"Area: ";
std::cout<<Radius * Radius * 3.14;
std::cout<<" square metres"<<std::endl;
}