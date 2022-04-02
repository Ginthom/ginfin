#include <prehdr.h>
#include <entry.h>

int main(int argc, char** argv) {
	float input;
	std::cin >> input;

	Entry entry = Entry();
	entry.set_amount(input);
	float output = entry.get_amount();

	std::cout << "Output: " << output << std::endl;
}
