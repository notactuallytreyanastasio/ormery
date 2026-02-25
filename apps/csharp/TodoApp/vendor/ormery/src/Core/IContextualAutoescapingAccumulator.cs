using C = Ormery.Core;
namespace Ormery.Core
{
    public interface IContextualAutoescapingAccumulator<CONTEXT__27, ESC__28> where CONTEXT__27: IContext where ESC__28: IEscaper
    {
        protected AutoescState<CONTEXT__27> State
        {
            get;
            set;
        }
        IEscaperPicker<CONTEXT__27, ESC__28> EscaperPicker
        {
            get;
        }
        IContextPropagator<CONTEXT__27> ContextPropagator
        {
            get;
        }
        protected static ESC__28 PrepareForAppendDefault(IContextualAutoescapingAccumulator<CONTEXT__27, ESC__28> this__33)
        {
            AfterPropagate<CONTEXT__27> after__166 = C::CoreGlobal.PropagateOver(this__33.ContextPropagator, this__33.State, null);
            AutoescState<CONTEXT__27> t___467 = after__166.StateAfter;
            this__33.State = t___467;
            string adjusted__167 = after__166.AdjustedString;
            if (!string.IsNullOrEmpty(adjusted__167)) this__33.CollectFixed(adjusted__167);
            return this__33.EscaperPicker.EscaperFor(this__33.State);
        }
        ESC__28 PrepareForAppend()
        {
            return PrepareForAppendDefault(this);
        }
        protected static void AppendSafeDefault(IContextualAutoescapingAccumulator<CONTEXT__27, ESC__28> this__34, string known__169)
        {
            AfterPropagate<CONTEXT__27> after__171 = C::CoreGlobal.PropagateOver(this__34.ContextPropagator, this__34.State, known__169);
            AutoescState<CONTEXT__27> t___458 = after__171.StateAfter;
            this__34.State = t___458;
            string adjusted__172 = after__171.AdjustedString;
            if (!string.IsNullOrEmpty(adjusted__172)) this__34.CollectFixed(adjusted__172);
        }
        void AppendSafe(string known__169)
        {
            AppendSafeDefault(this, known__169);
        }
        protected void CollectFixed(string fixedFragment__174);
    }
}
