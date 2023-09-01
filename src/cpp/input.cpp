#include "cxxgen.h"
#include "input.h"
#include <iostream>
#include <vector>
#include <string>

using namespace std;

void jurassic() {
    go_extinct();
}

void print()
{
   vector<string> msg {"Hello", "C++", "World", "from", "VS Code", "and the C++ extension!"};

   for (const string& word : msg)
   {
      cout << word << " ";
   }
   
   cout << endl;
}