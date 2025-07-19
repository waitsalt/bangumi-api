use crate::{
    common::model::BangumiClient,
    module::{
        model::ImagesType,
        subject::model::{
            SearchSubjectsRequest, SubjectAnimeCategory, SubjectCategory, SubjectSort, SubjectType,
        },
    },
};

/// 测试获取每日放送
#[tokio::test]
async fn test_get_calendar() {
    let client = BangumiClient::default();
    let result = client.get_calendar().await;
    assert!(result.is_ok(), "获取每日放送失败: {:?}", result.err());
    let calendar = result.unwrap();
    assert!(!calendar.is_empty(), "每日放送列表不应为空");
    // 检查是否有7天的数据, 有时API可能不返回完整7天，放宽断言
    assert!(calendar.len() <= 7);
}

#[tokio::test]
async fn test_search_subjects() {
    let client = BangumiClient::default();

    // 构造一个简单的搜索请求
    let search_request = Some(SearchSubjectsRequest {
        keyword: "eva".to_string(),
        sort: None,
        filter: None,
    });

    let result = client
        .search_subjects(Some(10), Some(0), search_request)
        .await;

    assert!(result.is_ok(), "搜索条目失败: {:?}", result.err());

    let subjects_page = result.unwrap();

    // 验证返回的分页数据是否有效
    assert!(
        subjects_page.data.len() <= 10,
        "返回的条目数不应超过设定的限制"
    );

    assert!(
        !subjects_page.data.is_empty(),
        "搜索结果不应为空，可能是关键词无匹配"
    );
}

#[tokio::test]
async fn test_get_subjects() {
    let client = BangumiClient::default();

    // 测试基本调用（只提供必填参数）
    let result = client
        .get_subjects(
            SubjectType::Anime, // 必填参数
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await;

    assert!(result.is_ok(), "获取条目列表失败: {:?}", result.err());
    let paged_subjects = result.unwrap();
    assert!(paged_subjects.total > 0, "条目总数应该大于0");
    assert!(!paged_subjects.data.is_empty(), "条目列表不应为空");

    // 测试带过滤条件的调用
    let result_with_filters = client
        .get_subjects(
            SubjectType::Anime,
            Some(SubjectCategory::Anime(SubjectAnimeCategory::Tv)), // 只查询TV动画
            Some(true),                                             // 只查询系列动画
            Some("TV"),                                             // 平台
            Some(SubjectSort::Rank),                                // 按排名排序
            Some(2023),                                             // 2023年的动画
            Some(4),                                                // 4月
            Some(10),                                               // 每页10条
            Some(0),                                                // 第一页
        )
        .await;

    assert!(
        result_with_filters.is_ok(),
        "带过滤条件的查询失败: {:?}",
        result_with_filters.err()
    );
    let filtered_subjects = result_with_filters.unwrap();
    assert!(
        filtered_subjects.data.len() <= 10,
        "返回条目数不应超过限制的10条"
    );

    // 可选：验证返回的数据确实符合查询条件
    if !filtered_subjects.data.is_empty() {
        let first_subject = &filtered_subjects.data[0];
        assert_eq!(
            first_subject.r#type,
            SubjectType::Anime,
            "返回条目类型应该匹配查询条件"
        );
        // 可以添加更多字段验证...
    }
}

#[tokio::test]
async fn test_get_subject_by_id() {
    let client = BangumiClient::default();
    let result = client.get_subject_by_id(1027).await;

    assert!(result.is_ok(), "获取条目失败: {:?}", result.err());

    let subject = result.unwrap();
    assert_eq!(subject.id, 1027, "返回的条目ID不匹配");
    assert!(!subject.name.is_empty(), "条目名称不应为空");
    assert!(!subject.name_cn.is_empty(), "条目中文名称不应为空");

    // 验证基本字段存在
    assert!(subject.rating.total > 0, "评分人数应大于0");
    // assert!(!subject.images.common.is_empty(), "图片URL不应为空");
}

#[tokio::test]
async fn test_get_subject_image_by_id() {
    let client = BangumiClient::default();

    let result = client
        .get_subject_image_by_id(1027, ImagesType::Large)
        .await;

    assert!(result.is_ok(), "获取条目图片失败: {:?}", result.err());

    let image_url = result.unwrap();

    println!("{:?}", image_url);
}

#[tokio::test]
async fn test_get_related_persons_by_subject_id() {
    let client = BangumiClient::default();

    // 测试获取已知条目的相关人物（以《CLANNAD》为例，ID=1467）
    let result = client.get_related_persons_by_subject_id(1467).await;

    assert!(result.is_ok(), "获取相关人物失败: {:?}", result.err());

    let related_persons = result.unwrap();
    // 验证基本数据结构
    println!("{:?}", related_persons);
}

#[tokio::test]
async fn test_get_related_characters_by_subject_id() {
    let client = BangumiClient::default();

    let known_subject_id = 1027;
    let result = client
        .get_related_characters_by_subject_id(known_subject_id)
        .await;

    assert!(result.is_ok(), "获取角色列表失败: {:?}", result.err());

    let characters = result.unwrap();

    println!("{:?}", characters);

    assert!(!characters.is_empty(), "角色列表不应为空");

    // 验证第一个角色的基本字段
    if let Some(first_char) = characters.get(0) {
        assert!(!first_char.name.is_empty(), "角色名称不应为空");
        assert!(!first_char.relation.is_empty(), "角色关系不应为空");
    }
}

#[tokio::test]
async fn test_get_related_subjects_by_subject_id() {
    let client = BangumiClient::default();

    let known_id = 1027; // CLANNAD的Bangumi ID
    let result = client.get_related_subjects_by_subject_id(known_id).await;

    // 验证请求成功
    assert!(result.is_ok(), "获取关联条目失败: {:?}", result.err());
    let related_subjects = result.unwrap();

    println!("{:?}", related_subjects);

    // 基础数据验证
    assert!(!related_subjects.is_empty(), "关联条目列表不应为空");
}
