using Microsoft.AspNetCore.Mvc;
using Microsoft.AspNetCore.Mvc.RazorPages;
using Microsoft.EntityFrameworkCore;
using TodoApp.Data;
using TodoApp.Models;

namespace TodoApp.Pages;

public class ListModel : PageModel
{
    private readonly AppDbContext _db;

    public ListModel(AppDbContext db)
    {
        _db = db;
    }

    public TodoList? TodoList { get; set; }
    public int? EditingTodoId { get; set; }

    public async Task OnGetAsync(int id, int? editTodoId)
    {
        TodoList = await _db.Lists
            .Include(l => l.Todos)
            .FirstOrDefaultAsync(l => l.Id == id);
        EditingTodoId = editTodoId;
    }

    public async Task<IActionResult> OnPostAddTodoAsync(int id, string title)
    {
        if (string.IsNullOrWhiteSpace(title))
            return RedirectToPage(new { id });

        var list = await _db.Lists.FindAsync(id);
        if (list == null)
            return RedirectToPage("/Index");

        _db.Todos.Add(new TodoItem
        {
            Title = title.Trim(),
            Completed = false,
            ListId = id,
            CreatedAt = DateTime.UtcNow
        });
        await _db.SaveChangesAsync();
        return RedirectToPage(new { id });
    }

    public async Task<IActionResult> OnPostToggleTodoAsync(int id, int todoId)
    {
        var todo = await _db.Todos.FindAsync(todoId);
        if (todo != null)
        {
            todo.Completed = !todo.Completed;
            await _db.SaveChangesAsync();
        }
        return RedirectToPage(new { id });
    }

    public async Task<IActionResult> OnPostUpdateTodoAsync(int id, int todoId, string title)
    {
        var todo = await _db.Todos.FindAsync(todoId);
        if (todo != null && !string.IsNullOrWhiteSpace(title))
        {
            todo.Title = title.Trim();
            await _db.SaveChangesAsync();
        }
        return RedirectToPage(new { id });
    }

    public async Task<IActionResult> OnPostDeleteTodoAsync(int id, int todoId)
    {
        var todo = await _db.Todos.FindAsync(todoId);
        if (todo != null)
        {
            _db.Todos.Remove(todo);
            await _db.SaveChangesAsync();
        }
        return RedirectToPage(new { id });
    }
}
