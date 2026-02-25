using System.ComponentModel.DataAnnotations;

namespace TodoApp.Models;

public class TodoList
{
    public int Id { get; set; }

    [Required]
    [StringLength(100)]
    public string Name { get; set; } = string.Empty;

    public DateTime CreatedAt { get; set; } = DateTime.UtcNow;

    public List<TodoItem> Todos { get; set; } = new();
}
