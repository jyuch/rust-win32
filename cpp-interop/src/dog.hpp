class Dog {
public:
    enum status {
        STATUS_WALKING,
        STATUS_STOP,
        STATUS_EATING
    };

private:
    const char *_name;
    enum status _status;

public:
    Dog(const char *name);
    ~Dog();
    void walk();
    void stop();
    enum status status() const { return _status; }
};