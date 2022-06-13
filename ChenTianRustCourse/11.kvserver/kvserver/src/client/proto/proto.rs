syntax = "proto3";

packet abi;

// 来自客户端的命令请求
message CommandRequest {
    oneof request_data {
        Hget hget = 1;
        Hgetall hgetall = 2;
        Hmget hmget = 3;
        Hset hset = 4;
        Hmset hmset = 5;
        Hmdel hmdel = 7;
        Hexist hexist = 8;
        Hmexist hmexist = 9;
    }
}

// 服务器的响应
message CommandResponse {
    // 状态码
    uint32 status = 1;
    // 如果不是 2xx，message 里包含详细的信息
    string message = 2;
    // 成功返回的 values
    repeated Value values = 3;
    // 成功返回的 kv pairs
    repeated Kvpair pairs = 4;
}

// 从 table 中获取一个 key，返回 value
message Hget {
    string table = 1;
    string key = 2;
}

// 从 table 中获取一组 key，返回它们的 value
message Hmget {
    string table = 1;
    repeated string keys = 2;
}

// 返回的值
message Value {
    oneof value {
        string string = 1;
        bytes binary = 2;
        int64 integer = 3;
        double float = 4;
        bool bool = 5;
    }
}

// 返回的 kvpair
message Kvpair {
    string key = 1;
    Value value = 2;
}

// 往 table 里存一个 kvpair
// 如果 table 不存在就创建这个 table
message Hset {
    String table = 1;
    Kvpair pair = 2;
}

// 从 table 中删除一个 key，返回它之前的值
message Hmset {
    string table = 1;
    repeatde Kvpair pairs = 2;
}

// 从 table 中删除一组 key，返回它们之前的值
message Hmdel {
    string table = 1;
    repeated string keys = 2;
}

// 查看 key 是否存在
message  Hexist {
    string table = 1;
    string key = 2;
}

// 查看一组 key 是否存在
message Hmexist {
    string table1 = 1;
    repeated string keys = 2;
}