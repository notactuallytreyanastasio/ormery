package com.ormery.todo;

import com.ormery.todo.model.TodoItem;
import com.ormery.todo.model.TodoList;
import com.ormery.todo.repository.TodoItemRepository;
import com.ormery.todo.repository.TodoListRepository;
import org.springframework.boot.CommandLineRunner;
import org.springframework.stereotype.Component;

@Component
public class DataLoader implements CommandLineRunner {

    private final TodoListRepository listRepository;
    private final TodoItemRepository itemRepository;

    public DataLoader(TodoListRepository listRepository, TodoItemRepository itemRepository) {
        this.listRepository = listRepository;
        this.itemRepository = itemRepository;
    }

    @Override
    public void run(String... args) {
        if (listRepository.count() > 0) {
            return;
        }

        // Seed list 1: Work Tasks
        TodoList work = new TodoList("Work Tasks");
        listRepository.save(work);
        itemRepository.save(new TodoItem("Review pull requests", work));
        itemRepository.save(new TodoItem("Update project documentation", work));
        itemRepository.save(new TodoItem("Fix login page bug", work));
        TodoItem done1 = new TodoItem("Set up CI pipeline", work);
        done1.setCompleted(true);
        itemRepository.save(done1);

        // Seed list 2: Groceries
        TodoList groceries = new TodoList("Groceries");
        listRepository.save(groceries);
        itemRepository.save(new TodoItem("Milk", groceries));
        itemRepository.save(new TodoItem("Eggs", groceries));
        itemRepository.save(new TodoItem("Bread", groceries));
        TodoItem done2 = new TodoItem("Coffee beans", groceries);
        done2.setCompleted(true);
        itemRepository.save(done2);
        TodoItem done3 = new TodoItem("Butter", groceries);
        done3.setCompleted(true);
        itemRepository.save(done3);
    }
}
