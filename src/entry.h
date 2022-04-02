#include <prehdr.h>

class Entry{
float amount;
std::string entry_time;

public:
	Entry();
	void save(std::string path);	
	void set_time(int hour, int day, int month, int year);
	void set_amount(float amount);	
	std::string get_time();
	float get_amount();

};
