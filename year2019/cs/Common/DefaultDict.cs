namespace Common;

public class DefaultDict<TKey, TValue> : Dictionary<TKey, TValue> where TValue : new() where TKey : notnull
{
    public new TValue this[TKey key]
    {
        get
        {
            if (!TryGetValue(key, out var val))
            {
                val = new TValue();
                Add(key, val);
            }
            return val;
        }
        set { base[key] = value; }
    }

}