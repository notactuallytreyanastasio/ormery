using C = Ormery.Core;
using T = System.Text;
namespace Ormery.Html
{
    public class SafeHtmlBuilder: C::IContextualAutoescapingAccumulator<HtmlEscaperContext, IHtmlEscaper>
    {
        public static T::StringBuilder NewCollector()
        {
            return new T::StringBuilder();
        }
        public static C::AutoescState<HtmlEscaperContext> InitialState()
        {
            return new C::AutoescState<HtmlEscaperContext>(new HtmlEscaperContext(0, 0, 0, 0), null);
        }
        public static HtmlContextPropagator Propagator()
        {
            return new HtmlContextPropagator();
        }
        public static C::IEscaperPicker<HtmlEscaperContext, IHtmlEscaper> Picker()
        {
            return new HtmlEscaperPicker();
        }
        public static SafeHtml FromCollector(T::StringBuilder collector__633)
        {
            return new SafeHtml(collector__633.ToString());
        }
        public static C::AutoescState<HtmlEscaperContext> MergeStates(C::AutoescState<HtmlEscaperContext> a__636, C::AutoescState<HtmlEscaperContext> b__637)
        {
            return a__636;
        }
        C::AutoescState<HtmlEscaperContext> _state__639;
        readonly T::StringBuilder collector__640;
        protected C::AutoescState<HtmlEscaperContext> State
        {
            get
            {
                return this._state__639;
            }
            set
            {
                this._state__639 = value;
            }
        }
        public C::IEscaperPicker<HtmlEscaperContext, IHtmlEscaper> EscaperPicker
        {
            get
            {
                return SafeHtmlBuilder.Picker();
            }
        }
        public C::IContextPropagator<HtmlEscaperContext> ContextPropagator
        {
            get
            {
                return SafeHtmlBuilder.Propagator();
            }
        }
        public SafeHtml Accumulated
        {
            get
            {
                return SafeHtmlBuilder.FromCollector(this.collector__640);
            }
        }
        protected void CollectFixed(string fixed__653)
        {
            this.collector__640.Append(fixed__653);
        }
        public void AppendSafeHtml(SafeHtml x__656)
        {
            string t___4702 = this.PrepareForAppend().ApplySafeHtml(x__656);
            this.collector__640.Append(t___4702);
        }
        public void AppendSafeUrl(SafeUrl x__659)
        {
            string t___4699 = this.PrepareForAppend().ApplySafeUrl(x__659);
            this.collector__640.Append(t___4699);
        }
        public void AppendInt32(int x__662)
        {
            string t___4696 = this.PrepareForAppend().ApplyInt32(x__662);
            this.collector__640.Append(t___4696);
        }
        public void AppendInt64(long x__665)
        {
            string t___4693 = this.PrepareForAppend().ApplyInt64(x__665);
            this.collector__640.Append(t___4693);
        }
        public void AppendFloat64(double x__668)
        {
            string t___4690 = this.PrepareForAppend().ApplyFloat64(x__668);
            this.collector__640.Append(t___4690);
        }
        public void AppendString(string x__671)
        {
            string t___4687 = this.PrepareForAppend().ApplyString(x__671);
            this.collector__640.Append(t___4687);
        }
        public SafeHtmlBuilder()
        {
            C::AutoescState<HtmlEscaperContext> t___4684 = SafeHtmlBuilder.InitialState();
            this._state__639 = t___4684;
            T::StringBuilder t___4685 = SafeHtmlBuilder.NewCollector();
            this.collector__640 = t___4685;
        }
        public IHtmlEscaper PrepareForAppend()
        {
            return C::IContextualAutoescapingAccumulator.PrepareForAppendDefault(this);
        }
        public void AppendSafe(string known___5756)
        {
            C::IContextualAutoescapingAccumulator.AppendSafeDefault(this, known___5756);
        }
    }
}
