# Task Record System (Rust + MySQL)

A command-line application in Rust that manages book records stored in MySQL. Offers full CRUD, sorting, and CSV export via a user-friendly menu.

## 🗂 Project Structure

```
![Описание изображения](https://github.com/m1rageLA/record-management-system/blob/main/task/images/mysql_start.png?raw=true)

## 📸 Screenshots
![MySQL Setup](https://github.com/m1rageLA/record-management-system/blob/main/task/images/mysql_start.png)
All interactions take place in the terminal. See below for each step in action.

| Step                          | Screenshot                                            |
|-------------------------------|-------------------------------------------------------|
| MySQL Setup                   | ![MySQL Setup](https://github.com/m1rageLA/record-management-system/blob/main/task/images/mysql_start.png?raw=true)           |
| Program Startup               | ![Program Start](task/images/program_start.png)       |
| Add a Record                  | ![Add Item](task/images/program_add_item.png)         |
| View All Records              | ![Show Items](task/images/program_show_items.png)     |
| Search by ID                  | ![Search](task/images/program_search.png)             |
| Update a Record               | ![Update](task/images/program_update.png)             |
| Delete a Record               | ![Delete](task/images/program_delete.png)             |
| Sort Records                  | ![Sort](task/images/program_sort.png)                 |
| Export to CSV (CLI prompt)    | ![Export](task/images/program_export.png)             |
| View CSV File (in editor)     | ![CSV All](task/images/program_csv_all.png)           |
| Exit                          | ![Exit](task/images/program_exit.png)                 |

## ℹ️ Usage Flow

1. **MySQL Setup**  
   Execute `schema.sql` to create the `mylib` database and `books` table (see [MySQL Setup](task/images/mysql_start.png)).

2. **Start the Program**  
   Run `cargo run` to launch the interactive menu (see [Program Start](task/images/program_start.png)).

3. **Add a Record**  
   Choose **Add**, fill in `ID`, `Title`, `Author`, `Year` (see [Add Item](task/images/program_add_item.png)).

4. **View All Records**  
   Choose **List** to display every record in the database (see [Show Items](task/images/program_show_items.png)).

5. **Search**  
   Choose **Search**, enter an `ID` to locate a book (see [Search](task/images/program_search.png)).

6. **Update**  
   Choose **Update**, enter an `ID`, then provide new values or leave blank to keep existing (see [Update](task/images/program_update.png)).

7. **Delete**  
   Choose **Delete**, enter an `ID` to remove it (see [Delete](task/images/program_delete.png)).

8. **Sort**  
   Choose **Sort by Title** or **Sort by Year** to reorder your list (see [Sort](task/images/program_sort.png)).

9. **Export to CSV**  
   Choose **Export CSV** to generate `books.csv` in the project root (see [Export](task/images/program_export.png)), then open it in your editor to view (see [CSV All](task/images/program_csv_all.png)).

10. **Exit**  
    Choose **Exit** to quit the application (see [Exit](task/images/program_exit.png)).

---

Happy coding! Feel free to file issues or pull requests for enhancements.

```
