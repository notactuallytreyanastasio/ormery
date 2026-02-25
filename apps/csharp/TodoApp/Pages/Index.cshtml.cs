using Microsoft.AspNetCore.Mvc;
using Microsoft.AspNetCore.Mvc.RazorPages;
using Microsoft.EntityFrameworkCore;
using TodoApp.Data;
using TodoApp.Models;

namespace TodoApp.Pages;

public class IndexModel : PageModel
{
    private readonly AppDbContext _db;

    public IndexModel(AppDbContext db)
    {
        _db = db;
    }

    public List<TodoList> Lists { get; set; } = new();
    public int? EditingListId { get; set; }

    public async Task OnGetAsync(int? editId)
    {
        Lists = await _db.Lists
            .Include(l => l.Todos)
            .OrderBy(l => l.CreatedAt)
            .ToListAsync();
        EditingListId = editId;
    }

    public async Task<IActionResult> OnPostCreateAsync(string name)
    {
        if (string.IsNullOrWhiteSpace(name))
            return RedirectToPage();

        _db.Lists.Add(new TodoList { Name = name.Trim(), CreatedAt = DateTime.UtcNow });
        await _db.SaveChangesAsync();
        return RedirectToPage();
    }

    public async Task<IActionResult> OnPostUpdateAsync(int id, string name)
    {
        var list = await _db.Lists.FindAsync(id);
        if (list == null)
            return RedirectToPage();

        if (!string.IsNullOrWhiteSpace(name))
        {
            list.Name = name.Trim();
            await _db.SaveChangesAsync();
        }
        return RedirectToPage();
    }

    public async Task<IActionResult> OnPostDeleteAsync(int id)
    {
        var list = await _db.Lists.FindAsync(id);
        if (list != null)
        {
            _db.Lists.Remove(list);
            await _db.SaveChangesAsync();
        }
        return RedirectToPage();
    }
}
