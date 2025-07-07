#include <iostream>
#include <string>

int main()
{
    int choice; // объявляем вне цикла

    do
    {
        std::string text =
            "1. Add record\n"
            "2. Search record\n"
            "3. Update record\n"
            "4. Delete record\n"
            "5. Exit\n"
            "Enter your choice: ";

        std::cout << text;
        std::cin >> choice;

        // можно сюда добавить обработку выбора
        switch (choice) {
            case 1:
                std::cout << "Adding record...\n";
                break;
            case 2:
                std::cout << "Searching...\n";
                break;
            case 3:
                std::cout << "Updating...\n";
                break;
            case 4:
                std::cout << "Deleting...\n";
                break;
            case 5:
                std::cout << "Exiting...\n";
                break;
            default:
                std::cout << "Invalid choice.\n";
        }

    } while (choice != 5); // пока не выбрал выход

    return 0;
}
