import {
  InMemoryStore, OrderClause, Query, WhereClause, field, schema, toInsertSql, toSqlQuery
} from "../ormery.js";
import {
  Test as Test_1675
} from "@temperlang/std/testing";
import {
  mapConstructor as mapConstructor_1792, pairConstructor as pairConstructor_1793
} from "@temperlang/core";
it("toSql: select all", function () {
    const test_1674 = new Test_1675();
    try {
      const s_1676 = schema("users", Object.freeze([field("name", "String", false, false)]));
      const store_1677 = new InMemoryStore();
      const q_1678 = new Query(s_1676, store_1677);
      const actual_1679 = q_1678.toSql().toString();
      let t_1680 = actual_1679 === "SELECT * FROM users";
      function fn_1681() {
        return "expected q.toSql().toString() == (" + "SELECT * FROM users" + ") not (" + actual_1679 + ")";
      }
      test_1674.assert(t_1680, fn_1681);
      return;
    } finally {
      test_1674.softFailToHard();
    }
});
it("toSql: select columns", function () {
    const test_1682 = new Test_1675();
    try {
      const s_1683 = schema("users", Object.freeze([field("name", "String", false, false), field("age", "Int", false, false)]));
      const store_1684 = new InMemoryStore();
      const q_1685 = new Query(s_1683, store_1684).select(Object.freeze(["name", "age"]));
      const actual_1686 = q_1685.toSql().toString();
      let t_1687 = actual_1686 === "SELECT name, age FROM users";
      function fn_1688() {
        return "expected q.toSql().toString() == (" + "SELECT name, age FROM users" + ") not (" + actual_1686 + ")";
      }
      test_1682.assert(t_1687, fn_1688);
      return;
    } finally {
      test_1682.softFailToHard();
    }
});
it("toSql: where string", function () {
    const test_1689 = new Test_1675();
    try {
      const s_1690 = schema("users", Object.freeze([field("name", "String", false, false)]));
      const store_1691 = new InMemoryStore();
      const q_1692 = new Query(s_1690, store_1691).where("name", "=", "Alice");
      const actual_1693 = q_1692.toSql().toString();
      let t_1694 = actual_1693 === "SELECT * FROM users WHERE name = 'Alice'";
      function fn_1695() {
        return "expected q.toSql().toString() == (" + "SELECT * FROM users WHERE name = 'Alice'" + ") not (" + actual_1693 + ")";
      }
      test_1689.assert(t_1694, fn_1695);
      return;
    } finally {
      test_1689.softFailToHard();
    }
});
it("toSql: where int", function () {
    const test_1696 = new Test_1675();
    try {
      const s_1697 = schema("users", Object.freeze([field("age", "Int", false, false)]));
      const store_1698 = new InMemoryStore();
      const q_1699 = new Query(s_1697, store_1698).where("age", "\u003e=", "18");
      const actual_1700 = q_1699.toSql().toString();
      let t_1701 = actual_1700 === "SELECT * FROM users WHERE age \u003e= 18";
      function fn_1702() {
        return "expected q.toSql().toString() == (" + "SELECT * FROM users WHERE age \u003e= 18" + ") not (" + actual_1700 + ")";
      }
      test_1696.assert(t_1701, fn_1702);
      return;
    } finally {
      test_1696.softFailToHard();
    }
});
it("toSql: SQL injection blocked", function () {
    const test_1703 = new Test_1675();
    try {
      const s_1704 = schema("users", Object.freeze([field("name", "String", false, false)]));
      const store_1705 = new InMemoryStore();
      const bobby_1706 = "Robert'); DROP TABLE users;--";
      const q_1707 = new Query(s_1704, store_1705).where("name", "=", "Robert'); DROP TABLE users;--");
      const result_1708 = q_1707.toSql().toString();
      const actual_1709 = result_1708;
      let t_1710 = actual_1709 === "SELECT * FROM users WHERE name = 'Robert''); DROP TABLE users;--'";
      function fn_1711() {
        return "expected result == (" + "SELECT * FROM users WHERE name = 'Robert''); DROP TABLE users;--'" + ") not (" + actual_1709 + ")";
      }
      test_1703.assert(t_1710, fn_1711);
      return;
    } finally {
      test_1703.softFailToHard();
    }
});
it("toSql: operator normalization", function () {
    const test_1712 = new Test_1675();
    try {
      const s_1713 = schema("users", Object.freeze([field("name", "String", false, false)]));
      const store_1714 = new InMemoryStore();
      const q_1715 = new Query(s_1713, store_1714).where("name", "==", "Alice");
      const actual_1716 = q_1715.toSql().toString();
      let t_1717 = actual_1716 === "SELECT * FROM users WHERE name = 'Alice'";
      function fn_1718() {
        return "expected q.toSql().toString() == (" + "SELECT * FROM users WHERE name = 'Alice'" + ") not (" + actual_1716 + ")";
      }
      test_1712.assert(t_1717, fn_1718);
      return;
    } finally {
      test_1712.softFailToHard();
    }
});
it("toSql: invalid operator fallback", function () {
    const test_1719 = new Test_1675();
    try {
      const s_1720 = schema("users", Object.freeze([field("name", "String", false, false)]));
      const store_1721 = new InMemoryStore();
      const q_1722 = new Query(s_1720, store_1721).where("name", "LIKE", "Alice");
      const actual_1723 = q_1722.toSql().toString();
      let t_1724 = actual_1723 === "SELECT * FROM users WHERE name = 'Alice'";
      function fn_1725() {
        return "expected q.toSql().toString() == (" + "SELECT * FROM users WHERE name = 'Alice'" + ") not (" + actual_1723 + ")";
      }
      test_1719.assert(t_1724, fn_1725);
      return;
    } finally {
      test_1719.softFailToHard();
    }
});
it("toSql: multiple where", function () {
    const test_1726 = new Test_1675();
    try {
      const s_1727 = schema("users", Object.freeze([field("age", "Int", false, false)]));
      const store_1728 = new InMemoryStore();
      const q_1729 = new Query(s_1727, store_1728).where("age", "\u003e=", "18").where("age", "\u003c", "30");
      const actual_1730 = q_1729.toSql().toString();
      let t_1731 = actual_1730 === "SELECT * FROM users WHERE age \u003e= 18 AND age \u003c 30";
      function fn_1732() {
        return "expected q.toSql().toString() == (" + "SELECT * FROM users WHERE age \u003e= 18 AND age \u003c 30" + ") not (" + actual_1730 + ")";
      }
      test_1726.assert(t_1731, fn_1732);
      return;
    } finally {
      test_1726.softFailToHard();
    }
});
it("toSql: order by", function () {
    const test_1733 = new Test_1675();
    try {
      const s_1734 = schema("users", Object.freeze([field("name", "String", false, false)]));
      const store_1735 = new InMemoryStore();
      const q_1736 = new Query(s_1734, store_1735).orderBy("name", "asc");
      const actual_1737 = q_1736.toSql().toString();
      let t_1738 = actual_1737 === "SELECT * FROM users ORDER BY name ASC";
      function fn_1739() {
        return "expected q.toSql().toString() == (" + "SELECT * FROM users ORDER BY name ASC" + ") not (" + actual_1737 + ")";
      }
      test_1733.assert(t_1738, fn_1739);
      return;
    } finally {
      test_1733.softFailToHard();
    }
});
it("toSql: order by desc", function () {
    const test_1740 = new Test_1675();
    try {
      const s_1741 = schema("users", Object.freeze([field("age", "Int", false, false)]));
      const store_1742 = new InMemoryStore();
      const q_1743 = new Query(s_1741, store_1742).orderBy("age", "desc");
      const actual_1744 = q_1743.toSql().toString();
      let t_1745 = actual_1744 === "SELECT * FROM users ORDER BY age DESC";
      function fn_1746() {
        return "expected q.toSql().toString() == (" + "SELECT * FROM users ORDER BY age DESC" + ") not (" + actual_1744 + ")";
      }
      test_1740.assert(t_1745, fn_1746);
      return;
    } finally {
      test_1740.softFailToHard();
    }
});
it("toSql: limit", function () {
    const test_1747 = new Test_1675();
    try {
      const s_1748 = schema("users", Object.freeze([field("name", "String", false, false)]));
      const store_1749 = new InMemoryStore();
      const q_1750 = new Query(s_1748, store_1749).limit(10);
      const actual_1751 = q_1750.toSql().toString();
      let t_1752 = actual_1751 === "SELECT * FROM users LIMIT 10";
      function fn_1753() {
        return "expected q.toSql().toString() == (" + "SELECT * FROM users LIMIT 10" + ") not (" + actual_1751 + ")";
      }
      test_1747.assert(t_1752, fn_1753);
      return;
    } finally {
      test_1747.softFailToHard();
    }
});
it("toSql: offset", function () {
    const test_1754 = new Test_1675();
    try {
      const s_1755 = schema("users", Object.freeze([field("name", "String", false, false)]));
      const store_1756 = new InMemoryStore();
      const q_1757 = new Query(s_1755, store_1756).offset(5);
      const actual_1758 = q_1757.toSql().toString();
      let t_1759 = actual_1758 === "SELECT * FROM users OFFSET 5";
      function fn_1760() {
        return "expected q.toSql().toString() == (" + "SELECT * FROM users OFFSET 5" + ") not (" + actual_1758 + ")";
      }
      test_1754.assert(t_1759, fn_1760);
      return;
    } finally {
      test_1754.softFailToHard();
    }
});
it("toSql: complex query", function () {
    const test_1761 = new Test_1675();
    try {
      const s_1762 = schema("users", Object.freeze([field("name", "String", false, false), field("age", "Int", false, false)]));
      const store_1763 = new InMemoryStore();
      const q_1764 = new Query(s_1762, store_1763).select(Object.freeze(["name", "age"])).where("age", "\u003e=", "18").orderBy("age", "desc").limit(10).offset(20);
      const actual_1765 = q_1764.toSql().toString();
      let t_1766 = actual_1765 === "SELECT name, age FROM users WHERE age \u003e= 18 ORDER BY age DESC LIMIT 10 OFFSET 20";
      function fn_1767() {
        return "expected q.toSql().toString() == (" + "SELECT name, age FROM users WHERE age \u003e= 18 ORDER BY age DESC LIMIT 10 OFFSET 20" + ") not (" + actual_1765 + ")";
      }
      test_1761.assert(t_1766, fn_1767);
      return;
    } finally {
      test_1761.softFailToHard();
    }
});
it("toSql: unicode escaping", function () {
    const test_1768 = new Test_1675();
    try {
      const s_1769 = schema("users", Object.freeze([field("name", "String", false, false)]));
      const store_1770 = new InMemoryStore();
      const q_1771 = new Query(s_1769, store_1770).where("name", "=", "Hello 世界");
      const actual_1772 = q_1771.toSql().toString();
      let t_1773 = actual_1772 === "SELECT * FROM users WHERE name = 'Hello 世界'";
      function fn_1774() {
        return "expected q.toSql().toString() == (" + "SELECT * FROM users WHERE name = 'Hello 世界'" + ") not (" + actual_1772 + ")";
      }
      test_1768.assert(t_1773, fn_1774);
      return;
    } finally {
      test_1768.softFailToHard();
    }
});
it("toSql: embedded quotes", function () {
    const test_1775 = new Test_1675();
    try {
      const s_1776 = schema("users", Object.freeze([field("name", "String", false, false)]));
      const store_1777 = new InMemoryStore();
      const q_1778 = new Query(s_1776, store_1777).where("name", "=", "O'Brien");
      const actual_1779 = q_1778.toSql().toString();
      let t_1780 = actual_1779 === "SELECT * FROM users WHERE name = 'O''Brien'";
      function fn_1781() {
        return "expected q.toSql().toString() == (" + "SELECT * FROM users WHERE name = 'O''Brien'" + ") not (" + actual_1779 + ")";
      }
      test_1775.assert(t_1780, fn_1781);
      return;
    } finally {
      test_1775.softFailToHard();
    }
});
it("toSql: empty string", function () {
    const test_1782 = new Test_1675();
    try {
      const s_1783 = schema("users", Object.freeze([field("name", "String", false, false)]));
      const store_1784 = new InMemoryStore();
      const q_1785 = new Query(s_1783, store_1784).where("name", "=", "");
      const actual_1786 = q_1785.toSql().toString();
      let t_1787 = actual_1786 === "SELECT * FROM users WHERE name = ''";
      function fn_1788() {
        return "expected q.toSql().toString() == (" + "SELECT * FROM users WHERE name = ''" + ") not (" + actual_1786 + ")";
      }
      test_1782.assert(t_1787, fn_1788);
      return;
    } finally {
      test_1782.softFailToHard();
    }
});
it("toInsertSql: basic insert", function () {
    const test_1789 = new Test_1675();
    try {
      const s_1790 = schema("users", Object.freeze([field("name", "String", false, false), field("age", "Int", false, false)]));
      const vals_1791 = mapConstructor_1792(Object.freeze([pairConstructor_1793("name", "Alice"), pairConstructor_1793("age", "25")]));
      const result_1794 = toInsertSql(s_1790, vals_1791);
      const actual_1795 = result_1794.toString();
      let t_1796 = actual_1795 === "INSERT INTO users (name, age) VALUES ('Alice', 25)";
      function fn_1797() {
        return "expected result.toString() == (" + "INSERT INTO users (name, age) VALUES ('Alice', 25)" + ") not (" + actual_1795 + ")";
      }
      test_1789.assert(t_1796, fn_1797);
      return;
    } finally {
      test_1789.softFailToHard();
    }
});
it("toInsertSql: injection blocked", function () {
    const test_1798 = new Test_1675();
    try {
      const s_1799 = schema("users", Object.freeze([field("name", "String", false, false)]));
      const vals_1800 = mapConstructor_1792(Object.freeze([pairConstructor_1793("name", "Robert'); DROP TABLE users;--")]));
      const result_1801 = toInsertSql(s_1799, vals_1800);
      const actual_1802 = result_1801.toString();
      let t_1803 = actual_1802 === "INSERT INTO users (name) VALUES ('Robert''); DROP TABLE users;--')";
      function fn_1804() {
        return "expected result.toString() == (" + "INSERT INTO users (name) VALUES ('Robert''); DROP TABLE users;--')" + ") not (" + actual_1802 + ")";
      }
      test_1798.assert(t_1803, fn_1804);
      return;
    } finally {
      test_1798.softFailToHard();
    }
});
it("toSqlQuery: standalone", function () {
    const test_1805 = new Test_1675();
    try {
      const s_1806 = schema("users", Object.freeze([field("name", "String", false, false), field("age", "Int", false, false)]));
      const result_1807 = toSqlQuery(s_1806, Object.freeze(["name"]), Object.freeze([new WhereClause("age", "\u003e", "21")]), Object.freeze([new OrderClause("name", "asc")]), 5, 0);
      const actual_1808 = result_1807.toString();
      let t_1809 = actual_1808 === "SELECT name FROM users WHERE age \u003e 21 ORDER BY name ASC LIMIT 5";
      function fn_1810() {
        return "expected result.toString() == (" + "SELECT name FROM users WHERE age \u003e 21 ORDER BY name ASC LIMIT 5" + ") not (" + actual_1808 + ")";
      }
      test_1805.assert(t_1809, fn_1810);
      return;
    } finally {
      test_1805.softFailToHard();
    }
});
it("toSql: adversarial field name blocked", function () {
    const test_1811 = new Test_1675();
    try {
      const s_1812 = schema("users", Object.freeze([field("name", "String", false, false)]));
      const store_1813 = new InMemoryStore();
      const q_1814 = new Query(s_1812, store_1813).where("1=1; DROP TABLE users; --", "=", "Alice");
      const actual_1815 = q_1814.toSql().toString();
      let t_1816 = actual_1815 === "SELECT * FROM users";
      function fn_1817() {
        return "expected q.toSql().toString() == (" + "SELECT * FROM users" + ") not (" + actual_1815 + ")";
      }
      test_1811.assert(t_1816, fn_1817);
      return;
    } finally {
      test_1811.softFailToHard();
    }
});
it("toSql: adversarial select column blocked", function () {
    const test_1818 = new Test_1675();
    try {
      const s_1819 = schema("users", Object.freeze([field("name", "String", false, false)]));
      const store_1820 = new InMemoryStore();
      const q_1821 = new Query(s_1819, store_1820).select(Object.freeze(["name", "1; DROP TABLE users"]));
      const actual_1822 = q_1821.toSql().toString();
      let t_1823 = actual_1822 === "SELECT name FROM users";
      function fn_1824() {
        return "expected q.toSql().toString() == (" + "SELECT name FROM users" + ") not (" + actual_1822 + ")";
      }
      test_1818.assert(t_1823, fn_1824);
      return;
    } finally {
      test_1818.softFailToHard();
    }
});
it("toSql: adversarial order by blocked", function () {
    const test_1825 = new Test_1675();
    try {
      const s_1826 = schema("users", Object.freeze([field("name", "String", false, false)]));
      const store_1827 = new InMemoryStore();
      const q_1828 = new Query(s_1826, store_1827).orderBy("1; DROP TABLE users", "asc");
      const actual_1829 = q_1828.toSql().toString();
      let t_1830 = actual_1829 === "SELECT * FROM users";
      function fn_1831() {
        return "expected q.toSql().toString() == (" + "SELECT * FROM users" + ") not (" + actual_1829 + ")";
      }
      test_1825.assert(t_1830, fn_1831);
      return;
    } finally {
      test_1825.softFailToHard();
    }
});
