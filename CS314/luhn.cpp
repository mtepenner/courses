// Matthew Penner
// CS314
// 05/27/2024

#include <iostream>
#include <bits/stdc++.h>
using namespace std;

bool isValid(const string & cardnumber) // Function to check if credit card numbers are valid, passing in a string of the credit card numbers
{
    int digits = cardnumber.length(); 

    int sum = 0, isSecond = false; 
    
    
    for(int i = digits - 1; i >=0; i--)
    {
        int d = cardnumber[i] - '0';

        if(isSecond == true)
        {
            d = d * 2;
        }


        sum += d / 10;
        sum += d % 10;

        isSecond = !isSecond;
    }
    return(sum % 10 == 0);
}


int main()
{

    string cardnumber;

    cout << "Enter your credit card number excluding spaces\n";
    cin >> cardnumber;
    bool passed = false;
    
    while(passed == false) 
    {
      if(isValid(cardnumber))
      {
          cout << "Hooray! Your credit card numbers are valid.\n";
          passed = true;
      }
  
      else
      {
          cout << "Invalid input, please try again.\n";
          cout << "Enter your credit card number excluding spaces\n";
          cin >> cardnumber;
      }
    }
    return 0;
}
