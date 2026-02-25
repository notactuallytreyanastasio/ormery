package com.ormery.todo.controller;

import com.ormery.todo.model.TodoItem;
import com.ormery.todo.model.TodoList;
import com.ormery.todo.repository.TodoItemRepository;
import com.ormery.todo.repository.TodoListRepository;
import org.springframework.stereotype.Controller;
import org.springframework.ui.Model;
import org.springframework.web.bind.annotation.*;

@Controller
public class TodoController {

    private final TodoListRepository listRepository;
    private final TodoItemRepository itemRepository;

    public TodoController(TodoListRepository listRepository, TodoItemRepository itemRepository) {
        this.listRepository = listRepository;
        this.itemRepository = itemRepository;
    }

    // --- Lists ---

    @GetMapping("/")
    public String index(Model model) {
        model.addAttribute("lists", listRepository.findAll());
        return "index";
    }

    @PostMapping("/lists")
    public String createList(@RequestParam String name) {
        if (name != null && !name.trim().isEmpty()) {
            listRepository.save(new TodoList(name.trim()));
        }
        return "redirect:/";
    }

    @PostMapping("/lists/{id}/delete")
    public String deleteList(@PathVariable Long id) {
        listRepository.deleteById(id);
        return "redirect:/";
    }

    // --- Single List with Todos ---

    @GetMapping("/lists/{id}")
    public String showList(@PathVariable Long id, Model model) {
        TodoList list = listRepository.findById(id).orElse(null);
        if (list == null) {
            return "redirect:/";
        }
        model.addAttribute("list", list);
        model.addAttribute("todos", itemRepository.findByListIdOrderByCreatedAtAsc(id));
        return "list";
    }

    @PostMapping("/lists/{id}/todos")
    public String addTodo(@PathVariable Long id, @RequestParam String title) {
        TodoList list = listRepository.findById(id).orElse(null);
        if (list != null && title != null && !title.trim().isEmpty()) {
            itemRepository.save(new TodoItem(title.trim(), list));
        }
        return "redirect:/lists/" + id;
    }

    // --- Todo Actions ---

    @PostMapping("/todos/{id}/toggle")
    public String toggleTodo(@PathVariable Long id) {
        TodoItem item = itemRepository.findById(id).orElse(null);
        if (item != null) {
            item.setCompleted(!item.getCompleted());
            itemRepository.save(item);
            return "redirect:/lists/" + item.getList().getId();
        }
        return "redirect:/";
    }

    @PostMapping("/todos/{id}/delete")
    public String deleteTodo(@PathVariable Long id) {
        TodoItem item = itemRepository.findById(id).orElse(null);
        if (item != null) {
            Long listId = item.getList().getId();
            itemRepository.delete(item);
            return "redirect:/lists/" + listId;
        }
        return "redirect:/";
    }

    @PostMapping("/todos/{id}/edit")
    public String editTodo(@PathVariable Long id, @RequestParam String title) {
        TodoItem item = itemRepository.findById(id).orElse(null);
        if (item != null && title != null && !title.trim().isEmpty()) {
            item.setTitle(title.trim());
            itemRepository.save(item);
            return "redirect:/lists/" + item.getList().getId();
        }
        return "redirect:/";
    }
}
