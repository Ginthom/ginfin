#include <entry.h>

Entry::Entry() {
	//do nothing for now
}

void Entry::save(std::string path) {
	//save entry to file at "path"
}

void Entry::set_time(int hour, int day, int month, int year) {
	//convert ints to one time string	
}

void Entry::set_amount(float amount) {
	Entry::amount = amount; 
}

std::string Entry::get_time() {
	//Get current time
	//Format to hour : moth.day.year
	//            hh : mm.dd.yy
	
	return "";
}

float Entry::get_amount() {
	return Entry::amount;
}
