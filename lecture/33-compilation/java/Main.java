public class Main {

    public static void main(String[] args) {
        System.out.println("Linked Lists!");
        
        Node list = Node.cons(1, Node.cons(2, Node.cons(3, null)));
        print_list(list);

        System.out.println("sum: " + sum(list));
    }

    public static void print_list(Node cursor) {
        while (cursor != null) {
            System.out.print(Node.first(cursor) + " -> ");
            cursor = Node.rest(cursor);
        }
        System.out.println("null");
    }

    public static int sum(Node cursor) {
        int sum = 0;
        while (cursor != null) {
            sum += Node.first(cursor);
            cursor = Node.rest(cursor);
        }
        return sum;
    }

}
