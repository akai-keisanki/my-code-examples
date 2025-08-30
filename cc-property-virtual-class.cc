template <typename T>
class Property
{
protected:
	
	T value;

public:

	Property (const T& value)
	: value(value)
	{}
	
	virtual operator T (void) const = 0;
	
	virtual Property& operator = (const T&) = 0;
};

class Widget
{
public:

	Widget () {}
	
	class IntegerVariable : public Property<int>
	{
	public:
	
		IntegerVariable (const int& value = 42)
		: Property<int>(value)
		{}
		
		operator int (void) const override final
		{
			return Property::value;
		}
		
		
		IntegerVariable& operator = (const int& value) override final
		{
			Property::value = value;
			return *this;
		}
	} integer_value;
};

signed main (void)
{
	Widget w;
	
	w.integer_value = 7;
	w.integer_value = w.integer_value * 7;
	w.integer_value = w.integer_value + 1;
	w.integer_value = w.integer_value / 2;

	return 0;
}
