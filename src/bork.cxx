#include <iostream>
#include <string>

class Bork {
public:
   Bork(std::string msg) {
      this->msg = msg;
   }
   void hiya() {
      std::cout << "Hiya" << this->msg << std::endl;
   }
private:
   std::string msg;
};


extern "C" {
   void bork() {
      auto lol = new Bork("haha");
      lol->hiya();
   }
}
