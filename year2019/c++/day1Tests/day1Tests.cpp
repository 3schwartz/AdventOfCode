#include "pch.h"
#include "CppUnitTest.h"
//#include "../day1/"

using namespace Microsoft::VisualStudio::CppUnitTestFramework;

namespace day1Tests
{
	int get_fuel(int mass) {
		return floor((mass / 3) - 2);
	}

	TEST_CLASS(day1Tests)
	{
	public:
		
		TEST_METHOD(TestCorrectFuelReturned)
		{
			// Arrange
			int expected = 1;

			// Act
			int actual = get_fuel(9);

			// Assert
			Assert::AreEqual(expected, actual);
		}
	};
}
